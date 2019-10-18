module Main exposing (..)

import Task
import Browser.Dom exposing (getViewportOf, setViewportOf)
import Browser
import Browser.Events exposing (onKeyDown)
import InfiniteList
import Debounce exposing (Debounce)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Ipc exposing (IpcMsg, Mode, RobotState)
import Json.Decode as D
import Ui exposing (..)
import Model exposing (..)
import Ports exposing (..)

main =
    Browser.element { init = init, update = update, subscriptions = subscriptions, view = view }

init : () -> ( Model, Cmd Msg )
init _ =
    ({ teamNumber = "", debounce = Debounce.init, enabled = False, estopped = False, mode = Ipc.Autonomous, alliance = Ipc.Red 1, stdout = [], stdoutList = InfiniteList.init, listScrollBottom = 0.0, robotState = { commsAlive = False, codeAlive = False, voltage = 0.0 }, activePage = Control },
       updateBackend <| Ipc.encodeMsg <| Ipc.UpdateTeamNumber { team_number = 0 })

debounceConfig : Debounce.Config Msg
debounceConfig = { strategy = Debounce.later 1000, transform = Debounced }

save : Int -> Cmd msg
save teamNumber = updateBackend <| Ipc.encodeMsg <| Ipc.UpdateTeamNumber { team_number = teamNumber }

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        EnableChange enabled -> ({ model | enabled = enabled }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateEnableStatus { enabled = enabled })
        ModeChange mode -> let shouldDisable = model.enabled in
                           if shouldDisable then
                               ({ model | mode = mode, enabled = False }, Cmd.batch [ updateBackend <| Ipc.encodeMsg <| Ipc.UpdateMode { mode = mode },
                                                                                  updateBackend <| Ipc.encodeMsg <| Ipc.UpdateEnableStatus { enabled = False } ])
                           else ({ model | mode = mode }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateMode { mode = mode })
        Debounced m ->
            let (debounce, cmd) = Debounce.update debounceConfig (Debounce.takeLast save) m model.debounce
            in ({ model | debounce = debounce }, cmd)
        BackendMessage m -> case m of
            Ipc.RobotStateUpdate state -> ({ model | robotState = state }, Cmd.none)
            Ipc.NewStdout { message } -> ({ model | stdout = (model.stdout ++ [message] )}, getViewportOf "stdoutListView" |> Task.andThen (\info -> setViewportOf "stdoutListView" 0 info.scene.height) |> Task.attempt (\_ -> Nop))
            _ -> (model, Cmd.none)
        StartStdoutWindow -> (model, updateBackend <| Ipc.encodeMsg <| Ipc.InitStdout { contents = model.stdout })
        InfiniteListMsg list -> ({ model | stdoutList = list }, getViewportOf "stdoutListView" |> Task.andThen (\info -> setViewportOf "stdoutListView" 0 info.scene.height) |> Task.attempt (\_ -> Nop))
        GlobalKeyboardEvent i -> if i == 13 && model.enabled then
                               ({ model | enabled = False }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateEnableStatus { enabled = False })
                           else (model, Cmd.none)
        Nop  -> (model, Cmd.none)
        ChangePage page -> ({ model | activePage = page }, case page of
            -- Scroll of stdout resets when tab is changed, send this command to re-reset it to what we want
            Control -> getViewportOf "stdoutListView" |> Task.andThen (\info -> setViewportOf "stdoutListView" 0 info.scene.height) |> Task.attempt (\_ -> Nop)
            Config -> Cmd.none)
        TeamNumberChange team ->
            if String.length team <= 4 then
                case String.toInt team of
                    Just teamNumber ->
                        if teamNumber > 0 then
                            let (debounce, cmd) = Debounce.push debounceConfig teamNumber model.debounce
                            in
                            ({ model | teamNumber = team, debounce = debounce }, cmd)
                        else (model, Cmd.none)
                    Nothing ->
                        if String.isEmpty team then
                        let (debounce, cmd) = Debounce.push debounceConfig 0 model.debounce
                        in
                            ({ model | teamNumber = "", debounce = debounce }, cmd)
                        else
                            (model, Cmd.none)
            else (model, Cmd.none) -- No teams above 9999 yet

view : Model -> Html Msg
view model =
    div []
    [
      ul [class "nav", class "nav-tabs" ] [
        li [ class "nav-item" ] [ a [ href "#", class "nav-link", if model.activePage == Control then class "active" else class "",
                                      onClick <| ChangePage Control ] [ text "Control" ] ],
        li [ class "nav-item" ] [ a [ href "#", class "nav-link", if model.activePage == Config then class "active" else class "",
                                      onClick <| ChangePage Config ] [ text "Config" ] ]
      ],
      case model.activePage of
          Control -> controlTab model
          Config -> configTab model
    ]

keyCodeDecoder : D.Decoder Msg
keyCodeDecoder = D.field "keyCode" D.int |> D.map(\i -> GlobalKeyboardEvent i)

subscriptions : Model -> Sub Msg
subscriptions _ = Sub.batch [ updateFrontend (\msg ->
                                        let m = D.decodeValue Ipc.decodeMsg msg
                                        in case m of
                                          Ok packet -> BackendMessage packet
                                          Err e -> BackendMessage <| Ipc.Invalid "Error")
                             , onKeyDown keyCodeDecoder ]

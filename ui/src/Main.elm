port module Main exposing (..)

import Task
import Browser.Dom exposing (getViewportOf, setViewportOf)
import Browser
import InfiniteList
import Debounce exposing (Debounce)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Ipc exposing (IpcMsg, Mode, RobotState)
import Json.Decode as D
import Json.Encode as E
import Ui exposing (..)
import Model exposing (..)

main =
    Browser.element { init = init, update = update, subscriptions = subscriptions, view = view }


port updateBackend : E.Value -> Cmd msg
port updateFrontend : (D.Value -> msg) -> Sub msg

init : () -> ( Model, Cmd Msg )
init _ =
    ({ teamNumber = "", debounce = Debounce.init, enabled = False, mode = Ipc.Autonomous, alliance = Ipc.Red 1, stdout = List.repeat 15 "", stdoutList = InfiniteList.init, listScrollBottom = 0.0, robotState = { commsAlive = False, codeAlive = False, voltage = 0.0 }, activePage = Control },
       Cmd.batch [ updateBackend <| Ipc.encodeMsg <| Ipc.UpdateTeamNumber { team_number = 0 }
                 , getViewportOf "stdoutListView"
                 |> Task.andThen (\info -> setViewportOf "stdoutListView" 0 (Debug.log "Scene height" info.scene.height))
                 |> Task.attempt (\_ -> Nop) ])


debounceConfig : Debounce.Config Msg
debounceConfig = { strategy = Debounce.later 1000, transform = Debounced }

save : Int -> Cmd msg
save teamNumber = updateBackend <| Ipc.encodeMsg <| Ipc.UpdateTeamNumber { team_number = teamNumber }

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        EnableChange enabled -> ({ model | enabled = enabled }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateEnableStatus { enabled = enabled })
        ModeChange mode -> ({ model | mode = mode }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateMode { mode = mode })
        Debounced m ->
            let (debounce, cmd) = Debounce.update debounceConfig (Debounce.takeLast save) m model.debounce
            in ({ model | debounce = debounce }, cmd)
        BackendMessage m -> case m of
            Ipc.RobotStateUpdate state -> ({ model | robotState = state }, Cmd.none)
            Ipc.NewStdout { message } -> ({ model | stdout = (model.stdout ++ [message] )}, Cmd.none)
            _ -> (model, Cmd.none)
        InfiniteListMsg list -> ({ model | stdoutList = list }, getViewportOf "stdoutListView" |> Task.andThen (\info -> setViewportOf "stdoutListView" 0 info.scene.height) |> Task.attempt (\_ -> Nop))
        Nop  -> (model, Cmd.none)
        ChangePage page -> ({ model | activePage = page }, Cmd.none)
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

subscriptions : Model -> Sub Msg
subscriptions _ = updateFrontend (\msg ->
                                        let m = D.decodeValue Ipc.decodeMsg msg
                                        in case m of
                                          Ok packet -> BackendMessage packet
                                          Err e -> BackendMessage <| Ipc.Invalid <| Debug.toString e)

module Main exposing (..)

import Browser
import Browser.Dom exposing (blur, getViewportOf, setViewportOf)
import Browser.Events exposing (onKeyDown)
import Dict
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode as D
import Lib.Ipc as Ipc exposing (IpcMsg, Mode, RobotState)
import Lib.Model as Model exposing (..)
import Lib.Ports exposing (..)
import Lib.Ui exposing (..)
import Task


main =
    Browser.element { init = init, update = update, subscriptions = subscriptions, view = view }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Model.initModel
    , updateBackend <| Ipc.encodeMsg <| Ipc.UpdateTeamNumber { team_number = 0 }
    )


updateTeamNumber : Int -> Cmd msg
updateTeamNumber teamNumber =
    updateBackend <| Ipc.encodeMsg <| Ipc.UpdateTeamNumber { team_number = teamNumber }

updateGSM : String -> Cmd msg
updateGSM gsm =
    updateBackend <| Ipc.encodeMsg <| Ipc.UpdateGSM { gsm = gsm }

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        EnableChange enabled ->
            let
                extra =
                    if enabled then
                        blur "enableButton"

                    else
                        blur "disableButton"
            in
            if model.estopped then
                ( model, Cmd.none )

            else
                ( { model | enabled = enabled }, Cmd.batch [ updateBackend <| Ipc.encodeMsg <| Ipc.UpdateEnableStatus { enabled = enabled }, extra |> Task.attempt (\_ -> Nop) ] )

        ModeChange mode ->
            let
                shouldDisable =
                    model.enabled
            in
            if shouldDisable then
                ( { model | mode = mode, enabled = False }
                , Cmd.batch
                    [ updateBackend <| Ipc.encodeMsg <| Ipc.UpdateMode { mode = mode }
                    , updateBackend <| Ipc.encodeMsg <| Ipc.UpdateEnableStatus { enabled = False }
                    ]
                )

            else
                ( { model | mode = mode }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateMode { mode = mode } )

        BackendMessage m ->
            case m of
                Ipc.RobotStateUpdate state ->
                    ( { model | robotState = state }, Cmd.none )

                Ipc.NewStdout { message } ->
                    ( { model | stdout = model.stdout ++ [ message ] }, getViewportOf "stdoutListView" |> Task.andThen (\info -> setViewportOf "stdoutListView" 0 info.scene.height) |> Task.attempt (\_ -> Nop) )

                Ipc.JoystickUpdate { removed, name } ->
                    case removed of
                        True ->
                            ( { model
                                | joysticks = List.filter (\s -> s /= name) model.joysticks
                                , joystickMappings = Dict.filter (\_ -> \s -> s /= name) model.joystickMappings
                              }
                            , Cmd.none
                            )

                        False ->
                            ( { model | joysticks = model.joysticks ++ [ name ] }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        InfiniteListMsg list ->
            ( { model | stdoutList = list }
            , getViewportOf "stdoutListView"
                |> Task.andThen
                    (\info ->
                        setViewportOf "stdoutListView"
                            info.viewport.x
                            (if model.enabled then
                                info.scene.height

                             else
                                info.viewport.y
                            )
                    )
                |> Task.attempt (\_ -> Nop)
            )

        SideViewChange maybe ->
            ( { model | explaining = maybe }, Cmd.none )

        AllianceStationChange alliance ->
            ( { model | alliance = alliance }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateAllianceStation { station = alliance } )

        RequestClick req ->
            ( model, updateBackend <| Ipc.encodeMsg <| Ipc.Request { req = req } )

        GlobalKeyboardEvent i ->
            case i of
                13 ->
                    case model.activePage of
                        Control ->
                            if model.enabled then
                                ( { model | enabled = False }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateEnableStatus { enabled = False } )

                            else
                                ( model, Cmd.none )

                        Config ->
                            ( model, Cmd.batch [ updateTeamNumber <| (model.teamNumber |> String.toInt |> Maybe.withDefault 0), updateGSM <| model.gsm ] )

                        JoysticksPage ->
                            ( model, Cmd.none )

                32 ->
                    ( { model | estopped = True }, updateBackend <| Ipc.encodeMsg <| Ipc.EstopRobot )

                _ ->
                    ( model, Cmd.none )

        Nop ->
            ( model, Cmd.none )

        ChangePage page ->
            ( { model | activePage = page }
            , case page of
                -- Scroll of stdout resets when tab is changed, send this command to re-reset it to what we want
                Control ->
                    getViewportOf "stdoutListView" |> Task.andThen (\info -> setViewportOf "stdoutListView" 0 info.scene.height) |> Task.attempt (\_ -> Nop)

                _ ->
                    Cmd.none
            )

        JoystickMappingUpdate n name ->
            let
                updatedJoysticks =
                    Dict.insert n name model.joystickMappings
                        |> Dict.filter
                            (\n2 ->
                                \s ->
                                    if s == name then
                                        n2 == n

                                    else
                                        True
                            )
            in
            ( { model | joystickMappings = updatedJoysticks }, updateBackend <| Ipc.encodeMsg <| Ipc.UpdateJoystickMapping { name = name, pos = n } )

        GSMChange gsm ->
            if String.length gsm <= 3 then
                ({model | gsm = gsm }, Cmd.none)
            else (model, Cmd.none)

        TeamNumberChange team ->
            if String.length team <= 4 then
                case String.toInt team of
                    Just teamNumber ->
                        if teamNumber > 0 then
                            ( { model | teamNumber = team }, Cmd.none )

                        else
                            ( model, Cmd.none )

                    Nothing ->
                        ( { model | teamNumber = "" }, Cmd.none )

            else
                ( model, Cmd.none )



-- No teams above 9999 yet


view : Model -> Html Msg
view model =
    div []
        [ ul [ class "nav", class "nav-tabs" ]
            [ li [ class "nav-item" ]
                [ a
                    [ href "#"
                    , class "nav-link"
                    , if model.activePage == Control then
                        class "active"

                      else
                        class ""
                    , onClick <| ChangePage Control
                    ]
                    [ text "Control" ]
                ]
            , li [ class "nav-item" ]
                [ a
                    [ href "#"
                    , class "nav-link"
                    , if model.activePage == Config then
                        class "active"

                      else
                        class ""
                    , onClick <| ChangePage Config
                    ]
                    [ text "Config" ]
                ]
            , li [ class "nav-item" ]
                [ a
                    [ href "#"
                    , class "nav-link"
                    , if model.activePage == JoysticksPage then
                        class "active"

                      else
                        class ""
                    , onClick <| ChangePage JoysticksPage
                    ]
                    [ text "Joysticks" ]
                ]
            ]
        , case model.activePage of
            Control ->
                controlTab model

            Config ->
                configTab model

            JoysticksPage ->
                joysticksTab model
        ]


keyCodeDecoder : D.Decoder Msg
keyCodeDecoder =
    D.field "keyCode" D.int |> D.map (\i -> GlobalKeyboardEvent i)


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.batch
        [ updateFrontend
            (\msg ->
                let
                    m =
                        D.decodeValue Ipc.decodeMsg msg
                in
                case m of
                    Ok packet ->
                        BackendMessage packet

                    Err e ->
                        BackendMessage <| Ipc.Invalid "Error"
            )
        , onKeyDown keyCodeDecoder
        ]

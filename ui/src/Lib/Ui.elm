module Lib.Ui exposing (..)

import Dict
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput, onMouseLeave, onMouseOver)
import InfiniteList
import Lib.Errors as Errors exposing (..)
import Lib.Ipc as Ipc exposing (AllianceStation, Mode, RobotState, allianceToS, modeToS)
import Lib.Model as Model exposing (..)
import Round



{- Creates the text of the status string given the current model -}


robotStatus : Model -> String
robotStatus model =
    if model.estopped then
        "Emergency Stopped"

    else if model.robotState.codeAlive then
        Ipc.modeToS model.mode ++ "\n"
            ++ (if model.enabled then
                    "Enabled"

                else
                    "Disabled"
               )

    else if not model.robotState.codeAlive && model.robotState.commsAlive then
        "No Robot Code"

    else
        "No Robot Communication"



{- Creates the red/green status pills associated with various aspects of the driver station's functionality.
   For systems that are currently not working, extra attributes can be supplied to display troubleshooting tips in the stdout view.
-}
telemetryBadge : List (Attribute Msg) -> String -> Bool -> Html Msg
telemetryBadge attrs caption alive =
    li [ class "list-group-item d-flex justify-content-between align-items-center py-2" ]
        [ text caption
        , if alive then
            span [ class "badge badge-success", style "color" "#00BC8C" ] [ text "AA" ]

          else
            let
                allAttrs =
                    List.append attrs [ class "badge badge-danger", style "color" "#E74C3C" ]
            in
            span allAttrs [ text "AA" ]
        ]


infiniteListConfig : InfiniteList.Config String Msg
infiniteListConfig =
    InfiniteList.config
        { itemView = itemView
        , itemHeight = InfiniteList.withConstantHeight 20
        , containerHeight = 500
        }
        |> InfiniteList.withOffset 300



{- View for stdout list -}
itemView : Int -> Int -> String -> Html Msg
itemView _ _ item =
    div [] [ text item ]



{- Determines the colour class that should be used to display voltage, depending on the current level -}
voltageColour : Float -> String
voltageColour voltage =
    if voltage >= 8.5 && voltage <= 11.5 then
        "text-warning"

    else if voltage < 8.5 then
        "text-danger"

    else
        "text-success"



{- Creates buttons to change the robot's mode -}
modeItem : Model -> Mode -> Html Msg
modeItem model mode =
    a
        [ class "list-group-item"
        , class "list-group-item-action"
        , class "py-1"
        , href "#"
        , if model.mode == mode then
            class "active"

          else
            class ""
        , onClick <| Model.ModeChange mode
        ]
        [ text <| modeToS mode ]



{- Creates a dropdown item corresponding to the given AllianceStation that will update the driver station when selected. -}
allianceStationItem : AllianceStation -> Html Msg
allianceStationItem alliance =
    a [ class "dropdown-item", class "py-1", href "#", onClick <| AllianceStationChange alliance ] [ text <| allianceToS alliance ]



{- Creates a joystick dropdown entry that will update the internal mapping when selected. -}
joystickEntry : Int -> String -> Html Msg
joystickEntry n name =
    a [ class "dropdown-item", href "#", onClick <| JoystickMappingUpdate n name ] [ text name ]



{- Creates the full view associated with remapping a joystick to some number n. -}
joystickRemapItem : Int -> Model -> Html Msg
joystickRemapItem n model =
    li [ class "list-group-item" ]
        [ div [ class "input-group" ]
            [ div [ class "input-group-prepend" ]
                [ span [ class "input-group-text" ] [ text <| String.fromInt n ++ ": " ]
                ]
            , div [ class "dropdown" ]
                [ button [ class "btn btn-secondary dropdown-toggle", type_ "button", attribute "data-toggle" "dropdown", attribute "aria-haspopup" "true", attribute "aria-extended" "false" ]
                    [ let
                        label =
                            Dict.get n model.joystickMappings |> Maybe.withDefault "Controller"
                      in
                      text label
                    ]
                , div [ class "dropdown-menu" ] <| List.map (joystickEntry n) model.joysticks
                ]
            ]
        ]


joysticks : Int -> Int -> Model -> List (Html Msg) -> List (Html Msg)
joysticks n end model l =
    if n == end then
        l

    else
        let
            l2 =
                l ++ [ joystickRemapItem (n - 1) model ]
        in
        joysticks (n - 1) end model l2



{- Builds up the full list of alliance stations a user can select (Red/Blue 1-3) -}
allianceStations : Int -> List (Html Msg) -> List (Html Msg)
allianceStations n l =
    case n of
        0 ->
            l

        _ ->
            if n > 3 then
                let
                    newL =
                        (allianceStationItem <| Ipc.Blue <| n - 3) :: l

                    newN =
                        n - 1
                in
                allianceStations newN newL

            else
                let
                    newL =
                        (allianceStationItem <| Ipc.Red n) :: l

                    newN =
                        n - 1
                in
                allianceStations newN newL



-- Different tabs
{- First tab that is displayed, contains mode information, alliance station selection, robot/joystick status, stdout view, and enable options. -}
controlTab : Model -> Html Msg
controlTab model =
    div [ class "container-fluid" ]
        [ div [ class "row" ]
            [ -- Mode selector
              div [ class "col-3", class "mt-4" ]
                [ div [ class "list-group" ]
                    [ modeItem model Ipc.Autonomous
                    , modeItem model Ipc.Teleoperated
                    , modeItem model Ipc.Test
                    ]
                ]
            , div [ class "col-3" ]
                [ ul [ class "list-group mt-4" ]
                    [ telemetryBadge [ onMouseOver <| SideViewChange <| Just Comms, onMouseLeave <| SideViewChange Nothing ] "Communications" model.robotState.commsAlive
                    , telemetryBadge [ onMouseOver <| SideViewChange <| Just Code, onMouseLeave <| SideViewChange Nothing ] "Robot Code" model.robotState.codeAlive
                    , telemetryBadge [ onMouseOver <| SideViewChange <| Just Joysticks, onMouseLeave <| SideViewChange Nothing ] "Joysticks" model.robotState.joysticks
                    ]
                ]
            , div [ class "col-2" ]
                [ p [ class "lead mt-4" ] [ text <| "Team # " ++ model.teamNumber ]
                , p [ class "text-center mt-4", class <| voltageColour model.robotState.voltage ] [ b [] [ text <| Round.round 1 model.robotState.voltage ++ "V" ] ]
                ]
            , div [ class "col" ]
                [ div
                    [ style "width" "330px"
                    , style "height" "200px"
                    , style "overflow-x" "auto"
                    , style "overflow-y" "auto"
                    , style "-webkit-overflow-scrolling" "touch"
                    , style "color" "#fff"
                    , class "form-control"
                    , class "bg-secondary"
                    , class "mt-4"
                    , id "stdoutListView"
                    , InfiniteList.onScroll InfiniteListMsg
                    ]
                    [ case model.explaining of
                        Just expl ->
                            case expl of
                                Comms ->
                                    InfiniteList.view infiniteListConfig model.stdoutList Errors.robotComms

                                Code ->
                                    InfiniteList.view infiniteListConfig model.stdoutList Errors.robotCode

                                Joysticks ->
                                    InfiniteList.view infiniteListConfig model.stdoutList Errors.joysticks

                        Nothing ->
                            InfiniteList.view infiniteListConfig model.stdoutList model.stdout
                    ]
                ]
            ]
        , div [ class "row ", style "margin-top" "-50px" ]
            [ -- Enable buttons
              div [ class "col-3 text-center", class "mt-4" ]
                [ div [ class "btn-group", attribute "role" "group", attribute "aria-label" "State Control Buttons" ]
                    [ button
                        [ id "enableButton"
                        , type_ "button"
                        , class "btn btn-lg "
                        , class "btn-success"
                        , if model.enabled then
                            class "active"

                          else
                            class ""
                        , onClick <| EnableChange True
                        ]
                        [ text "Enable" ]
                    , button
                        [ id "disableButton"
                        , type_ "button"
                        , class "btn btn-lg"
                        , class "btn-danger"
                        , if not model.enabled then
                            class "active"

                          else
                            class ""
                        , onClick <| EnableChange False
                        ]
                        [ text "Disable" ]
                    ]
                ]
            , -- Team station selector
              div [ class "col-3", class "mt-4" ]
                [ div [ class "input-group" ]
                    [ div [ class "input-group-prepend" ]
                        [ label [ for "teamSelectorDropdown", class "dropdown-label lead" ] [ text "Team Station " ]
                        ]
                    , div [ class "dropdown", id "teamSelectorDropdown" ]
                        [ button
                            [ class "btn"
                            , class "btn-secondary"
                            , class "dropdown-toggle"
                            , type_ "button"
                            , id "dropdownMenuButton"
                            , attribute "data-toggle" "dropdown"
                            , attribute "aria-haspopup" "true"
                            , attribute "aria-expanded" "false"
                            ]
                            [ text <| allianceToS model.alliance ]
                        , div [ class "dropdown-menu", class "py-1", attribute "aria-labelledby" "dropdownMenuButton" ] <|
                            allianceStations 6 []
                        ]
                    ]
                ]
            , div [ class "col-2 align-items-center" ]
                [ p [ class "text-center lead" ]
                    [ text <| robotStatus model ]
                ]
            , div [ class "col" ] []
            ]
        ]



{- Second tab in the driver station, contains text fields to update team number, game data (WIP), and buttons to restart code and roboRIO. -}


configTab : Model -> Html Msg
configTab model =
    div [ class "container" ]
        [ div [ class "row", class "align-items-center" ]
            [ div [ class "col" ]
                [ label [ for "teamNumberInput" ] [ text "Team Number" ]
                , div [ class "input-group", class "mb-3" ]
                    [ input [ type_ "number", class "form-control", id "teamNumberInput", value model.teamNumber, onInput TeamNumberChange ] []
                    ]
                , label [ for "gameDataInput" ] [ text "Game Data" ]
                , div [ class "input-group", class "mb-3" ]
                    [ input [ type_ "text", class "form-control disabled", id "gameDataInput" ] []
                    ]
                ]
            , div [ class "col" ] []
            , div [ class "col", class "pull-right" ]
                [ div [ class "btn-group-vertical" ]
                    [ button [ type_ "button", class "btn", class "btn-secondary", onClick <| RequestClick <| Ipc.RestartRoborio ] [ text "Restart roboRIO" ]
                    , button [ type_ "button", class "btn", class "btn-secondary", onClick <| RequestClick <| Ipc.RestartCode ] [ text "Restart Robot Code" ]
                    ]
                ]
            ]
        ]



{- Final tab shown, allows user to update the mappings used internally for joystick order when sending user input to the robot -}


joysticksTab : Model -> Html Msg
joysticksTab model =
    div [ class "container" ]
        [ div [ class "row" ]
            [ div [ class "col" ]
                [ ul [ class "list-group" ]
                    (List.reverse <| joysticks 3 0 model [])
                ]
            , div [ class "col" ]
                [ ul [ class "list-group" ]
                    (List.reverse <| joysticks 6 3 model [])
                ]
            ]
        ]

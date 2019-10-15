module Ui exposing (..)

import InfiniteList
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput, onSubmit)
import Model exposing (..)
import Ipc exposing (Mode, modeToS, AllianceStation, allianceToS)

telemetryBadge : String -> Bool -> Html Msg
telemetryBadge caption alive
    = li [ class "list-group-item d-flex justify-content-between align-items-center py-2" ]
      [
        text caption,
        span [ class "badge", class <| if alive then "badge-success" else "badge-danger" ] [ text <| if alive then "OK" else "ERR" ]
      ]

infiniteListConfig : InfiniteList.Config String Msg
infiniteListConfig =
    InfiniteList.config
        { itemView = itemView
        , itemHeight = InfiniteList.withConstantHeight 20
        , containerHeight = 500
        }
        |> InfiniteList.withOffset 300

itemView : Int -> Int -> String -> Html Msg
itemView _ _ item = div [] [ text item ]

modeItem : Model -> Mode -> Html Msg
modeItem model mode = a [ class "list-group-item", class "list-group-item-action", class "py-1",
  if model.mode == mode then class "active" else class "",
  onClick <| Model.ModeChange mode ] [ text <| modeToS mode ]

allianceStationItem : AllianceStation -> Html Msg
allianceStationItem alliance = a [ class "dropdown-item", class "py-1", href "#" ] [ text <| allianceToS alliance ]

allianceStations : Int -> List (Html Msg) -> List (Html Msg)
allianceStations n l = case n of
    0 -> l
    _ -> if n > 3 then
           let newL = (allianceStationItem <| Ipc.Blue <| n - 3) :: l
               newN = n - 1
            in
            allianceStations newN newL
        else
            let newL = (allianceStationItem <| Ipc.Red n) :: l
                newN = n - 1
            in
            allianceStations newN newL

-- Different tabs
controlTab : Model -> Html Msg
controlTab model =
      div [ class "container" ]
      [
        div [ class "row" ]
        [
          -- Mode selector
          div [ class "col", class "mt-4" ]
          [
            div [ class "list-group" ]
            [
              modeItem model Ipc.Autonomous,
              modeItem model Ipc.Teleoperated,
              modeItem model Ipc.Test
            ]
          ],
          div [ class "col" ]
          [
            ul [ class "list-group mt-4" ]
            [
              telemetryBadge "Communications" model.robotState.commsAlive,
              telemetryBadge "Robot Code" model.robotState.codeAlive,
              telemetryBadge "Joysticks" False
            ]
          ],
          div [ class "col" ]
          [
            p [ class "lead mt-4" ] [ text <| "Team # " ++ model.teamNumber ]
          ],
          div [ class "col" ]
          [
            div [
                  style "width" "100%",
                  style "height" "150px",
                  style "overflow-x" "hidden",
                  style "overflow-y" "auto",
                  style "-webkit-overflow-scrolling" "touch",
                  style "color" "#fff",
                  class "form-control",
                  class "bg-secondary",
                  class "mt-4",
                  id "stdoutListView",
                  InfiniteList.onScroll InfiniteListMsg
                ]
            [
              InfiniteList.view infiniteListConfig model.stdoutList model.stdout
            ]
          ]
        ],
        div [ class "row" ]
        [
          -- Enable buttons
          div [ class "col", class "mt-4" ]
          [
            div [ class "btn-group", attribute "role" "group", attribute "aria-label" "State Control Buttons" ]
            [
              button [ type_ "button", class "btn btn-lg", class "btn-success", if model.enabled then class "active" else class "",
                onClick <| EnableChange True
               ] [ text "Enable" ],
              button [ type_ "button", class "btn btn-lg", class "btn-danger", if not model.enabled then class "active" else class "",
                onClick <| EnableChange False
               ] [ text "Disable" ]
            ]
          ],
          -- Team station selector
          div [ class "col", class "mt-4" ]
          [
            div [ class "input-group" ]
            [
              div [ class "input-group-prepend" ]
              [
                label [ for "teamSelectorDropdown", class "dropdown-label lead" ] [ text "Team Station " ]
              ],
              div [ class "dropdown", id "teamSelectorDropdown" ]
              [
                button [ class "btn", class "btn-secondary", class "dropdown-toggle", type_ "button", id "dropdownMenuButton",
                         attribute "data-toggle" "dropdown", attribute "aria-haspopup" "true", attribute "aria-expanded" "false" ] [ text <| allianceToS model.alliance ],
                div [ class "dropdown-menu", class "py-1", attribute "aria-labelledby" "dropdownMenuButton" ]
                <| allianceStations 6 []
              ]
            ]
          ],
          div [ class "col" ]
          [
            h4 [ class "text-center" ]
            [ text ( Ipc.modeToS model.mode ++ "\n" ++ if model.enabled then "Enabled" else "Disabled" )]
          ],
          div [ class "col" ] []
        ]
      ]

configTab : Model -> Html Msg
configTab model
    = div [ class "container" ]
      [
        div [ class "row", class "align-items-center" ]
        [
          div [ class "col" ]
          [
            label [ for "teamNumberInput" ] [ text "Team Number"],
            div [ class "input-group", class "mb-3" ]
            [
              input [ type_ "number", class "form-control", id "teamNumberInput", value model.teamNumber, onInput TeamNumberChange ] []
            ],

            label [ for "gameDataInput" ] [ text "Game Data" ],
            div [ class "input-group", class "mb-3" ]
            [
              input [ type_ "text", class "form-control", id "gameDataInput" ] []
            ]
          ],
          div [ class "col" ] [],
          div [ class "col", class "pull-right" ]
          [
            div [ class "btn-group-vertical" ]
            [
                button [ type_ "button", class "btn", class "btn-secondary" ] [ text "Restart roboRIO" ],
                button [ type_ "button", class "btn", class "btn-secondary" ] [ text "Restart Robot Code" ]
            ]
          ]
        ]
      ]
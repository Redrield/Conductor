port module Main exposing (..)

import Browser
import InfiniteList
import Debounce exposing (Debounce)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Ipc exposing (IpcMsg, Mode, allianceToS)
import Json.Decode as D
import Json.Encode as E
import Ui exposing (..)
import Model exposing (..)

main =
    Browser.element { init = init, update = update, subscriptions = subscriptions, view = view }


port updateBackend : E.Value -> Cmd msg
port updateFrontend : (D.Value -> msg) -> Sub msg

init : () -> ( Model, Cmd msg )
init _ =
    ({ teamNumber = "", debounce = Debounce.init, enabled = False, mode = Ipc.Autonomous, alliance = Ipc.Red 1, stdout = [], stdoutList = InfiniteList.init, voltage = 0.0 }, Cmd.none)

debounceConfig : Debounce.Config Msg
debounceConfig = { strategy = Debounce.later 1000, transform = Debounced }

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
            Ipc.RobotStateUpdate { comms_alive, code_alive, voltage } -> ({ model | voltage = voltage }, Cmd.none)
            _ -> (model, Cmd.none)
        InfiniteListMsg list -> ({ model | stdoutList = list }, Cmd.none)
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
        -- TODO: Uptime
        ],
        div [ class "col" ]
        [
         -- TODO: Voltage, statuses
        ],
        div [ class "col" ]
        [
          -- TODO: stdout
        ]
      ],
      div [ class "row" ]
      [
        -- Enable buttons
        div [ class "col", class "mt-4" ]
        [
          div [ class "btn-group", attribute "role" "group", attribute "aria-label" "State Control Buttons" ]
          [
            button [ type_ "button", class "btn", class "btn-success", if model.enabled then class "active" else class "",
              onClick <| EnableChange True
             ] [ text "Enable" ],
            button [ type_ "button", class "btn", class "btn-danger", if not model.enabled then class "active" else class "",
              onClick <| EnableChange False
             ] [ text "Disable" ]
          ]
        ],
        -- Team station selector
        div [ class "col", class "mt-4" ]
        [
          label [ for "teamSelectorDropdown", class "dropdown-label" ] [ text "Team Station: " ],
          div [ class "dropdown", id "teamSelectorDropdown" ]
          [
            button [ class "btn", class "btn-secondary", class "dropdown-toggle", type_ "button", id "dropdownMenuButton",
                     attribute "data-toggle" "dropdown", attribute "aria-haspopup" "true", attribute "aria-expanded" "false" ] [ text <| allianceToS model.alliance ],
            div [ class "dropdown-menu", class "py-1", attribute "aria-labelledby" "dropdownMenuButton" ]
            <| allianceStations 6 []
          ]
        ],
        div [ class "col" ]
        [
          h4 [ class "text-center" ]
          [ text ( Ipc.modeToS model.mode ++ "\n" ++ if model.enabled then "Enabled" else "Disabled" )]
        ]
      ]
    ]

subscriptions : Model -> Sub Msg
subscriptions _ = updateFrontend (\msg -> let m = D.decodeValue Ipc.decodeMsg msg
                                        in case m of
                                          Ok packet -> BackendMessage packet
                                          Err _ -> BackendMessage Ipc.Invalid)

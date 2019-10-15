module Ui exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)
import Model exposing (..)
import Ipc exposing (Mode, modeToS, AllianceStation, allianceToS)

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

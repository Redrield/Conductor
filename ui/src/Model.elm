module Model exposing (..)

import Debounce exposing (Debounce)
import InfiniteList
import Ipc exposing (AllianceStation, Mode)

type alias Model =
    {
        teamNumber : String
    ,   debounce : Debounce Int
    ,   enabled : Bool
    ,   mode : Mode
    ,   alliance : AllianceStation
    ,   stdout : List String
    ,   stdoutList : InfiniteList.Model
    ,   voltage : Float
    }

type Msg
    = EnableChange Bool
    | ModeChange Mode
    | TeamNumberChange String
    | BackendMessage Ipc.IpcMsg
    | Debounced Debounce.Msg
    | InfiniteListMsg InfiniteList.Model

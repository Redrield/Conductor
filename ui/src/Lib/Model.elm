module Lib.Model exposing (..)

import Browser.Dom exposing (Error, Viewport)
import Debounce exposing (Debounce)
import Dict exposing (Dict)
import InfiniteList
import Lib.Ipc as Ipc exposing (AllianceStation, Mode, Request, RobotState, robotStateInit)

type alias Model =
    { teamNumber : String
    , debounce : Debounce Int
    , enabled : Bool
    , mode : Mode
    , alliance : AllianceStation
    , stdout : List String
    , stdoutList : InfiniteList.Model
    , listScrollBottom : Float
    , robotState : RobotState
    , activePage : ActivePage
    , explaining : Maybe ErrorExplanation
    , joysticks : List String
    , joystickMappings : Dict Int String
    , estopped : Bool
    }

initModel : Model
initModel
    = { teamNumber = ""
       , debounce = Debounce.init
       , enabled = False
       , estopped = False
       , mode = Ipc.Autonomous
       , alliance = Ipc.Red 1
       , stdout = []
       , joysticks = []
       , joystickMappings = Dict.empty
       , stdoutList = InfiniteList.init
       , listScrollBottom = 0.0
       , explaining = Nothing
       , robotState = robotStateInit
       , activePage = Control }

type ActivePage
    = Control
    | Config
    | JoysticksPage

type ErrorExplanation
    = Comms
    | Code
    | Joysticks

type Msg
    = EnableChange Bool
    | ModeChange Mode
    | AllianceStationChange AllianceStation
    | RequestClick Request
    | TeamNumberChange String
    | BackendMessage Ipc.IpcMsg
    | Debounced Debounce.Msg
    | InfiniteListMsg InfiniteList.Model
    | ChangePage ActivePage
    | GlobalKeyboardEvent Int
    | SideViewChange (Maybe ErrorExplanation)
    | JoystickMappingUpdate Int String
    | Nop

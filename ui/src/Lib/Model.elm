{- All model information for the application -}


module Lib.Model exposing (..)

import Browser.Dom exposing (Error, Viewport)
import Debounce exposing (Debounce)
import Dict exposing (Dict)
import InfiniteList
import Lib.Ipc as Ipc exposing (AllianceStation, Mode, Request, RobotState, robotStateInit)


type alias Model =
    { teamNumber : String -- Current contents of the team number text field, 0 if empty
    , enabled : Bool -- Whether the robot is enabled
    , mode : Mode -- The currently selected mode
    , alliance : AllianceStation -- The currently selected team station
    , stdout : List String -- All stdout data from the robot
    , stdoutList : InfiniteList.Model -- Associated InfiniteList helper
    , robotState : RobotState -- The last status packet received from the backend
    , activePage : ActivePage -- The currently selected tab of the DS
    , explaining : Maybe ErrorExplanation -- The current error explanation being displayed to the user, if any
    , joysticks : List String -- All joysticks available to the application
    , joystickMappings : Dict Int String -- Mapping of joystick number to joystick name
    , estopped : Bool -- Whether the robot is emergency stopped
    }



-- Initial model state


initModel : Model
initModel =
    { teamNumber = ""
    , enabled = False
    , estopped = False
    , mode = Ipc.Autonomous
    , alliance = Ipc.Red 1
    , stdout = []
    , joysticks = []
    , joystickMappings = Dict.empty
    , stdoutList = InfiniteList.init
    , explaining = Nothing
    , robotState = robotStateInit
    , activePage = Control
    }



{- The page (as selected in the nav view) whose contents should be displayed -}


type ActivePage
    = Control -- Robot control
    | Config -- Robot configuration
    | JoysticksPage -- Joystick configuration



{- The error whose explanation should be displayed in the stdout view -}


type ErrorExplanation
    = Comms -- Why is there no robot communication
    | Code -- Why is there no robot code
    | Joysticks -- Why are there no joysticks detected


type Msg
    = EnableChange Bool
    | ModeChange Mode
    | AllianceStationChange AllianceStation
    | RequestClick Request
    | TeamNumberChange String
    | BackendMessage Ipc.IpcMsg
    | InfiniteListMsg InfiniteList.Model
    | ChangePage ActivePage
    | GlobalKeyboardEvent Int
    | SideViewChange (Maybe ErrorExplanation)
    | JoystickMappingUpdate Int String
    | Nop

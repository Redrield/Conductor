{- Types and codecs used to communicate with the rust backend. All types are sent and received encoded as JSON -}


module Lib.Ipc exposing (..)

import Json.Decode as D exposing (Decoder, bool, field, float, int, string)
import Json.Encode as E exposing (object)


type Mode
    = Autonomous
    | Teleoperated
    | Test


type AllianceStation
    = Red Int
    | Blue Int


allianceToS : AllianceStation -> String
allianceToS a =
    case a of
        Red n ->
            "Red " ++ String.fromInt n

        Blue n ->
            "Blue " ++ String.fromInt n


modeToS : Mode -> String
modeToS m =
    case m of
        Autonomous ->
            "Autonomous"

        Teleoperated ->
            "Teleoperated"

        Test ->
            "Test"


type alias RobotState =
    { commsAlive : Bool
    , codeAlive : Bool
    , voltage : Float
    , joysticks : Bool
    }


robotStateInit : RobotState
robotStateInit =
    { commsAlive = False, codeAlive = False, voltage = 0.0, joysticks = False }


type Request
    = RestartRoborio
    | RestartCode


type IpcMsg
    = UpdateTeamNumber { team_number : Int }
    | UpdateMode { mode : Mode }
    | UpdateEnableStatus { enabled : Bool }
    | JoystickUpdate { removed : Bool, name : String }
    | UpdateJoystickMapping { name : String, pos : Int }
    | RobotStateUpdate RobotState
    | NewStdout { message : String }
    | Request { req : Request }
    | UpdateAllianceStation { station : AllianceStation }
    | EstopRobot
    | Invalid String


encodeAlliance : AllianceStation -> E.Value
encodeAlliance alliance =
    case alliance of
        Red n ->
            object
                [ ( "color", E.string "Red" )
                , ( "value", E.int n )
                ]

        Blue n ->
            object
                [ ( "color", E.string "Blue" )
                , ( "value", E.int n )
                ]


decodeMode : Decoder Mode
decodeMode =
    D.string
        |> D.andThen
            (\s ->
                case String.toLower s of
                    "autonomous" ->
                        D.succeed Autonomous

                    "teleoperated" ->
                        D.succeed Teleoperated

                    "test" ->
                        D.succeed Test

                    _ ->
                        D.fail <| "Invalid mode " ++ s
            )


encodeMode : Mode -> E.Value
encodeMode m =
    case m of
        Teleoperated ->
            E.string "Teleoperated"

        Autonomous ->
            E.string "Autonomous"

        Test ->
            E.string "Test"


encodeRequest : Request -> E.Value
encodeRequest req =
    case req of
        RestartRoborio ->
            E.string "RestartRoborio"

        RestartCode ->
            E.string "RestartCode"


encodeMsg : IpcMsg -> E.Value
encodeMsg msg =
    case msg of
        UpdateTeamNumber { team_number } ->
            object
                [ ( "type", E.string "UpdateTeamNumber" )
                , ( "team_number", E.int team_number )
                ]

        UpdateMode { mode } ->
            object
                [ ( "type", E.string "UpdateMode" )
                , ( "mode", encodeMode mode )
                ]

        UpdateEnableStatus { enabled } ->
            object
                [ ( "type", E.string "UpdateEnableStatus" )
                , ( "enabled", E.bool enabled )
                ]

        JoystickUpdate { removed, name } ->
            object
                [ ( "type", E.string "JoystickUpdate" )
                , ( "removed", E.bool removed )
                , ( "name", E.string name )
                ]

        UpdateJoystickMapping { name, pos } ->
            object
                [ ( "type", E.string "UpdateJoystickMapping" )
                , ( "name", E.string name )
                , ( "pos", E.int pos )
                ]

        RobotStateUpdate { commsAlive, codeAlive, voltage } ->
            object
                [ ( "type", E.string "RobotStateUpdate" )
                , ( "comms_alive", E.bool commsAlive )
                , ( "code_alive", E.bool codeAlive )
                , ( "voltage", E.float voltage )
                ]

        NewStdout { message } ->
            object
                [ ( "type", E.string "NewStdout" )
                , ( "message", E.string message )
                ]

        UpdateAllianceStation { station } ->
            object
                [ ( "type", E.string "UpdateAllianceStation" )
                , ( "station", encodeAlliance station )
                ]

        Request { req } ->
            object
                [ ( "type", E.string "Request" )
                , ( "req", encodeRequest req )
                ]

        EstopRobot ->
            object [ ( "type", E.string "EstopRobot" ) ]

        Invalid _ ->
            object []


decodeMsg : Decoder IpcMsg
decodeMsg =
    field "type" string
        |> D.andThen
            (\ty ->
                case ty of
                    "NewStdout" ->
                        field "message" string |> D.map (\msg -> NewStdout { message = msg })

                    -- There is probably a better way to do this, but i don't know it
                    "JoystickUpdate" ->
                        field "removed" bool |> D.andThen (\removed -> field "name" string |> D.map (\name -> JoystickUpdate { removed = removed, name = name }))

                    "RobotStateUpdate" ->
                        field "comms_alive" bool
                            |> D.andThen
                                (\comms ->
                                    field "code_alive" bool
                                        |> D.andThen
                                            (\code ->
                                                field "voltage" float
                                                    |> D.andThen
                                                        (\voltage ->
                                                            field "joysticks" bool |> D.map (\joysticks -> RobotStateUpdate { commsAlive = comms, codeAlive = code, voltage = voltage, joysticks = joysticks })
                                                        )
                                            )
                                )

                    _ ->
                        D.fail <| "no"
            )

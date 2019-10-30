{- Contains text lists that can be displayed to explain why some functionality of the driver station is not working.
   These lines are displayed in the stdout view when an error badge is hovered over
-}


module Lib.Errors exposing (..)


robotComms : List String
robotComms =
    [ "The robot controller and driver station are not able to communicate." ]


robotCode : List String
robotCode =
    [ "There is no user code running on the robot."
    , "1. Code may be crashing on startup, check robot console for potential error."
    , "2. There may be no code downloaded. Deploy your code to the robot."
    ]


joysticks : List String
joysticks =
    [ "No joysticks were identified"
    , "1. Ensure they are plugged in"
    , "2. Disconnect and reconnect the joysticks"
    ]

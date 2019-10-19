module Errors exposing (..)

robotComms : List String
robotComms
    = [ "The robot controller and driver station are not able to communicate."]

robotCode : List String
robotCode
    = [ "There is no user code running on the robot."
      , "1. If developing code, use the tools to start it"
      , "2. If ready for competition, build and deploy it as a startup application"
      ]

joysticks : List String
joysticks
    = [ "No joysticks were identified"
      , "1. Ensure they are plugged in"
      , "2. Disconnect and reconnect the joysticks"
      ]
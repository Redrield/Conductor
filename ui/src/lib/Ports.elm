port module Ports exposing (..)

import Json.Encode as E
import Json.Decode as D

port updateBackend : E.Value -> Cmd msg
port updateFrontend : (D.Value -> msg) -> Sub msg

{- Ports to communicate to/from Rust via webview -}


port module Lib.Ports exposing (..)

import Json.Decode as D
import Json.Encode as E



{- Send some encoded message to the backend via window.external.invoke -}


port updateBackend : E.Value -> Cmd msg



{- Receive some message from the backend via update() -}


port updateFrontend : (D.Value -> msg) -> Sub msg

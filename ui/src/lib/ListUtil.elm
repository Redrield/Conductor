module ListUtil exposing (..)

ensureCirc : Int -> a -> List a -> List a
ensureCirc len elem l
    = let listLen = List.length l
      in
      if listLen < len then
          l ++ [elem]
      else
          (List.tail l |> Maybe.withDefault []) ++ [elem]

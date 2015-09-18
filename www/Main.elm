import Html exposing (Html, ul, li, text)

type alias Model = {
  carryFlag: Bool,
  zeroFlag: Bool,
  interrupt: Bool,
  decimalMode: Bool,
  break: Bool,
  overflow: Bool,
  sign: Bool
}

boolToString : Bool -> String
boolToString val = 
  if val then "1" else "0"

view : Model -> Html
view model = 
  ul [] [
    li [] [ text <| boolToString model.carryFlag ],
    li [] [ text <| boolToString model.zeroFlag ],
    li [] [ text <| boolToString model.interrupt ],
    li [] [ text <| boolToString model.decimalMode ],
    li [] [ text <| boolToString model.break ],
    li [] [ text <| boolToString model.overflow ],
    li [] [ text <| boolToString model.sign ]
  ]

main =
  view { 
    carryFlag = True, 
    zeroFlag = False, 
    interrupt = True, 
    decimalMode = True,
    break = False,
    overflow = True,
    sign = True
 }

module Main exposing (main)

import Browser
import Html exposing (Html, div, text)
import Html.Attributes exposing (style)
import Svg exposing (Svg, svg, polygon, g)
import Svg.Attributes exposing (viewBox, points, fill, stroke, width, height, transform, preserveAspectRatio)
import Svg.Events


type alias Model =
    { selectedHex : Maybe ( Int, Int ) 
    , currentPlayer : Player
    , turnNumber : Int
    }

type Player 
    = White | Black

init : Model
init =
    { selectedHex = Nothing
    , currentPlayer = White 
    , turnNumber = 1
    }


type Msg
    = ClickedHex ( Int, Int )


update : Msg -> Model -> Model
update msg model =
    case msg of
        ClickedHex pos ->
            { model | selectedHex = Just pos }


view : Model -> Html Msg
view model =
    div [ style "position" "relative", style "width" "100vw", style "height" "100vh" ]
        [ boardView model 
        , topRightInfo model
        ]

topRightInfo : Model -> Html Msg
topRightInfo model = 
    div 
        [ style "position" "fixed"
        , style "top" "16px"
        , style "right" "16px"
        , style "background" "white"
        , style "border" "1px solid #ddd"
        , style "border-radius" "12px"
        , style "padding" "12x 24px"
        , style "display" "flex"
        , style "gap" "32px"
        ]
        [ infoColumn "Player" (playerLabel model.currentPlayer)
        , infoColumn "Turn" (String.fromInt model.turnNumber)
        ]

infoColumn : String -> String -> Html Msg
infoColumn label value = 
    div [ style "text-align" "center" ]
        [ div [ style "color" "#888", style "font-size" "14px" ] [ text label ]
        , div [ style "font-weight" "bold" ] [ text value ]
        ]

playerLabel : Player -> String
playerLabel player = 
    case player of 
        White -> "White"
        Black -> "Black"

boardView : Model -> Html Msg
boardView model =
    svg
        [ width "100%"
        , height "100%"
        , viewBox "-300 -300 600 600"
        , preserveAspectRatio "xMidYMid meet"
        , style "position" "absolute"
        , style "top" "0"
        , style "left" "0"
        ]
        (List.map (renderHex model) boardHexes)


boardHexes : List ( Int, Int )
boardHexes =
    List.concatMap
        (\q -> List.map (\r -> ( q, r )) (List.range -4 4))
        (List.range -4 4)
        |> List.filter (\( q, r ) -> abs (q + r) <= 4)


renderHex : Model -> ( Int, Int ) -> Svg Msg
renderHex model ( q, r ) =
    let
        ( x, y ) =
            axialToPixel q r 40

        isSelected =
            model.selectedHex == Just ( q, r )

        hexFill =
            if isSelected then
                "#8fe0a0"
            else
                "white"
    in
    g [ transform ("translate(" ++ String.fromFloat x ++ "," ++ String.fromFloat y ++ ")") ]
        [ polygon
            [ points (hexPoints 0 0 38)
            , fill hexFill
            , stroke "#ddd"
            , Svg.Events.onClick (ClickedHex ( q, r ))
            ]
            []
        ]


axialToPixel : Int -> Int -> Float -> ( Float, Float )
axialToPixel q r size =
    ( size * (sqrt 3 * toFloat q + sqrt 3 / 2 * toFloat r)
    , size * (3 / 2 * toFloat r)
    )


hexPoints : Float -> Float -> Float -> String
hexPoints cx cy size =
    List.range 0 5
        |> List.map (hexCorner cx cy size)
        |> List.map (\( x, y ) -> String.fromFloat x ++ "," ++ String.fromFloat y)
        |> String.join " "


hexCorner : Float -> Float -> Float -> Int -> ( Float, Float )
hexCorner cx cy size i =
    let
        angleRad =
            degrees (60 * toFloat i - 30)
    in
    ( cx + size * cos angleRad, cy + size * sin angleRad )


main : Program () Model Msg
main =
    Browser.sandbox { init = init, view = view, update = update }
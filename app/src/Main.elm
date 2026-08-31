module Main exposing (main)

import Browser
import Html exposing (Html, div, text)
import Html.Attributes exposing (style)
import Svg exposing (Svg, svg, polygon, g)
import Svg.Attributes exposing (viewBox, points, fill, stroke, width, height, transform, preserveAspectRatio)

type alias Model =
    { turnNumber : Int }


init : Model
init =
    { turnNumber = 1 }

type Msg
    = NoOp


update : Msg -> Model -> Model
update msg model =
    case msg of
        NoOp ->
            model


view : Model -> Html Msg
view model =
    div [ style "position" "relative", style "width" "100vm", style "height" "100vh"]
        [ boardView ]

boardView : Html Msg
boardView = 
    svg 
        [ width "100%"
        , height "100%"
        , viewBox "-300 -300 600 600"
        , preserveAspectRatio "xMidYMid meet"
        , style "position" "absolute"
        , style "top" "0"
        , style "left" "0"
        ]
        (List.map renderHex boardHexes)

boardHexes : List ( Int, Int )
boardHexes = 
    List.concatMap
        (\q -> List.map (\r -> ( q, r )) (List.range -4 4))
        (List.range -4 4)
        |> List.filter (\( q, r ) -> abs ( q + r ) <= 4)

renderHex : ( Int, Int ) -> Svg Msg
renderHex ( q, r ) =
    let
        ( x, y ) = 
            axialToPixel q r 40
    in
        g [ transform ("translate(" ++ String.fromFloat x ++ "," ++ String.fromFloat y ++ ")") ]
        [ polygon
            [ points (hexPoints 0 0 38)
            , fill "white"
            , stroke "#ddd"
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
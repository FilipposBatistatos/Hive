module Main exposing (main)

import Html exposing (Html)
import Svg exposing (Svg, svg, polygon)
import Svg.Attributes exposing (viewBox, points, fill, stroke, width, height)
import Svg exposing (polygon)

main : Html msg
main = 
    svg
        [ width "400"
        , height "400"
        , viewBox "0 0 400 400"
        ]
        [ hexagon 200 200]

hexagon : Float -> Float -> Svg msg
hexagon centerX centerY = 
    polygon
        [ points (hexPoint centerX centerY 50)
        , fill "white"
        , stroke "grey"
        ]
        []

hexPoint : Float -> Float -> Float -> String
hexPoint cx cy size = 
    List.range 0 5
        |> List.map(\i -> hexCorner cx cy size i)
        |> List.map(\( x, y ) -> String.fromFloat x ++ "," ++ String.fromFloat y)
        |> String.join " "

hexCorner : Float -> Float -> Float -> Int -> ( Float, Float )
hexCorner cx cy size i = 
    let 
        angleDeg = 
            60 * toFloat i - 30
        angleRad = 
            degrees angleDeg
    
    in
    ( cx + size * cos angleRad, cy + size * sin angleRad )

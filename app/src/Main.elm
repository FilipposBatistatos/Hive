port module Main exposing (main)

import Browser
import Html exposing (Html, div, text, button)
import Html.Attributes exposing (style)
import Html.Events
import Json.Decode as Decode
import Svg exposing (Svg, svg, polygon, g)
import Svg.Attributes exposing (viewBox, points, fill, stroke, width, height, transform, preserveAspectRatio)
import Svg.Events
import Json.Encode as Encode


port requestNewGame : () -> Cmd msg
port receiveInitialState : (Decode.Value -> msg) -> Sub msg
port requestLegalPlacements : Decode.Value -> Cmd msg
port receiveLegalPlacements : (Decode.Value -> msg) -> Sub msg
port requestApplyMove : ( Decode.Value, Decode.Value ) -> Cmd msg
port receiveNewState : ( Decode.Value -> msg ) -> Sub msg

-- TYPES

type Player
    = White
    | Black

type PieceKind
    = Bee
    | Spider
    | Beetle
    | Grasshopper
    | Ant

type alias Position =
    { q : Int, r : Int }

type alias Piece =
    { kind : PieceKind, owner : Player }

type alias Board =
    { stacks : List ( Position, List Piece ) }

type GameResult
    = Win Player
    | Draw

type alias Unplaced =
    List ( Player, List ( PieceKind, Int ) )

type alias GameState =
    { board : Board
    , turn : Player
    , turnNumber : Int
    , unplaced : Unplaced
    , result : Maybe GameResult
    }

type Move 
    = Place PieceKind Position
    | MovePiece Position Position


-- MODEL

type alias Model =
    { selectedHex : Maybe Position
    , selectedHandPiece : Maybe PieceKind
    , gameState : Maybe GameState
    , decodeErrorMsg : Maybe String
    , legalPlacements : List Position
    }


init : Model
init =
    { selectedHex = Nothing
    , selectedHandPiece = Nothing
    , gameState = Nothing
    , decodeErrorMsg = Nothing
    , legalPlacements = []
    }


-- MSG / UPDATE

type Msg
    = ClickedHex Position
    | ClickedHandPiece PieceKind
    | ClickedNewGame
    | GotInitialState Decode.Value
    | GotLegalPlacements Decode.Value
    | GotNewState Decode.Value

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ClickedHex pos ->
            case ( model.gameState, model.selectedHandPiece ) of
                ( Just state, Just kind ) -> 
                    ( model
                    , requestApplyMove ( encodeGameState state, encodeMove (Place kind pos) )
                    )   
                _ -> 
                    ( { model | selectedHex = Just pos }, Cmd.none )

        ClickedHandPiece kind ->
            case model.gameState of
                Just state ->
                    ( { model | selectedHandPiece = Just kind, selectedHex = Nothing }
                    , requestLegalPlacements (encodeGameState state) 
                    )
                Nothing ->
                    ( model, Cmd.none )

        ClickedNewGame ->
            ( model, requestNewGame () )

        GotInitialState value ->
            case Decode.decodeValue gameStateDecoder value of
                Ok state ->
                    ( { model | gameState = Just state, decodeErrorMsg = Nothing }, Cmd.none )

                Err error ->
                    ( { model | decodeErrorMsg = Just (Decode.errorToString error) }, Cmd.none )

        GotLegalPlacements value ->
            case Decode.decodeValue (Decode.list positionDecoder) value of
                Ok positions ->
                    ( { model | legalPlacements = positions }, Cmd.none )

                Err error ->
                    ( { model | decodeErrorMsg = Just (Decode.errorToString error) }, Cmd.none )
        
        GotNewState value -> 
            case Decode.decodeValue gameStateDecoder value of
                Ok state ->
                    ( { model
                        | gameState = Just state
                        , selectedHex = Nothing
                        , selectedHandPiece = Nothing
                        , legalPlacements = []
                        }
                    , Cmd.none 
                    )
                Err error -> 
                    ( { model | decodeErrorMsg = Just (Decode.errorToString error) }, Cmd.none )

subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.batch
        [ receiveInitialState GotInitialState
        , receiveLegalPlacements GotLegalPlacements
        , receiveNewState GotNewState
        ]

-- VIEW

view : Model -> Html Msg
view model =
    div [ style "position" "relative", style "width" "100vw", style "height" "100vh" ]
        [ boardView model
        , topLeftControls
        , topRightInfo model
        , handToolbar model
        , errorBanner model
        ]


errorBanner : Model -> Html Msg
errorBanner model =
    case model.decodeErrorMsg of
        Just msg ->
            div
                [ style "position" "fixed"
                , style "bottom" "0"
                , style "left" "0"
                , style "right" "0"
                , style "background" "#ffdddd"
                , style "color" "#900"
                , style "padding" "8px"
                , style "font-size" "12px"
                , style "white-space" "pre-wrap"
                ]
                [ text msg ]

        Nothing ->
            text ""


topLeftControls : Html Msg
topLeftControls =
    div [ style "position" "fixed", style "top" "16px", style "left" "16px" ]
        [ button
            [ Html.Events.onClick ClickedNewGame
            , style "padding" "10px 16px"
            , style "border-radius" "8px"
            , style "border" "1px solid #ddd"
            , style "background" "white"
            , style "font-size" "14px"
            , style "cursor" "pointer"
            ]
            [ text "New Game" ]
        ]


topRightInfo : Model -> Html Msg
topRightInfo model =
    case model.gameState of
        Nothing ->
            text ""

        Just state ->
            div
                [ style "position" "fixed"
                , style "top" "16px"
                , style "right" "16px"
                , style "background" "white"
                , style "border" "1px solid #ddd"
                , style "border-radius" "12px"
                , style "padding" "12px 24px"
                , style "display" "flex"
                , style "gap" "32px"
                ]
                [ infoColumn "Player" (playerLabel state.turn)
                , infoColumn "Turn" (String.fromInt state.turnNumber)
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


-- BOARD

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


boardHexes : List Position
boardHexes =
    List.concatMap
        (\q -> List.map (\r -> Position q r) (List.range -4 4))
        (List.range -4 4)
        |> List.filter (\pos -> abs (pos.q + pos.r) <= 4)


renderHex : Model -> Position -> Svg Msg
renderHex model pos =
    let
        ( x, y ) =
            axialToPixel pos.q pos.r 40

        pieceHere =
            model.gameState
                |> Maybe.andThen (\state -> lookupStack state.board pos)
                |> Maybe.andThen List.head

        isSelected =
            model.selectedHex == Just pos

        isLegalPlacement =
            List.member pos model.legalPlacements

        isClickable = 
            case model.selectedHandPiece of 
                Just _ -> 
                    isLegalPlacement

                Nothing -> 
                    True

        cursorStyle = 
            if isClickable then 
                Svg.Attributes.style "cursor: pointer;"
            else
                Svg.Attributes.style "cursor: default;"

        hexFill =
            case pieceHere of
                Just piece -> 
                    pieceColor piece.owner
                
                Nothing ->
                    if isSelected then
                        "#f0c283"
                    else if isLegalPlacement then
                        "#d4f0d9" -- lighter green: "you could place here", distinct from "selected"
                    else
                        "white"
        
        baseAttrs = 
            [ points (hexPoints 0 0 38)
            , fill hexFill
            , stroke "#ddd"
            , cursorStyle
            ]

        attrs = 
            if isClickable then 
                baseAttrs ++ [ Svg.Events.onClick ( ClickedHex pos ) ]
            else
                baseAttrs

    in
    g [ transform ("translate(" ++ String.fromFloat x ++ "," ++ String.fromFloat y ++ ")") ]
        (polygon attrs []
            :: (case pieceHere of 
                Just piece ->
                    [ Svg.text_
                        [ Svg.Attributes.textAnchor "middle"
                        , Svg.Attributes.dominantBaseline "central"
                        , Svg.Attributes.fontSize "28"
                        ]
                        [ Svg.text (pieceGlyph piece.kind) ]
                    ]

                Nothing ->
                    []
            )
        )

lookupStack : Board -> Position -> Maybe (List Piece)
lookupStack board pos =
    board.stacks
        |> List.filter (\( p, _ ) -> p == pos)
        |> List.head
        |> Maybe.map Tuple.second


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


-- HAND TOOLBAR

handToolbar : Model -> Html Msg
handToolbar model =
    case model.gameState of
        Nothing ->
            text ""

        Just state ->
            let
                myHand =
                    state.unplaced
                        |> List.filter (\( player, _ ) -> player == state.turn)
                        |> List.head
                        |> Maybe.map Tuple.second
                        |> Maybe.withDefault []
            in
            div
                [ style "position" "fixed"
                , style "bottom" "24px"
                , style "left" "50%"
                , style "transform" "translateX(-50%)"
                , style "background" "white"
                , style "border" "1px solid #ddd"
                , style "border-radius" "12px"
                , style "padding" "12px"
                , style "display" "flex"
                , style "gap" "12px"
                ]
                (List.map (handSlot model) myHand)


handSlot : Model -> ( PieceKind, Int ) -> Html Msg
handSlot model ( kind, count ) =
    let
        isSelected =
            model.selectedHandPiece == Just kind
    in
    div
        [ style "position" "relative"
        , style "width" "70px"
        , style "height" "70px"
        , style "border-radius" "8px"
        , style "border" (if isSelected then "2px solid #e05555" else "1px solid #ddd")
        , style "background" "#fdf8e8"
        , style "display" "flex"
        , style "align-items" "center"
        , style "justify-content" "center"
        , style "font-size" "28px"
        , style "cursor" "pointer"
        , Html.Events.onClick (ClickedHandPiece kind)
        ]
        [ text (pieceGlyph kind)
        , countBadge count
        ]


countBadge : Int -> Html Msg
countBadge count =
    if count > 1 then
        div
            [ style "position" "absolute"
            , style "top" "-8px"
            , style "right" "-8px"
            , style "background" "#5577dd"
            , style "color" "white"
            , style "border-radius" "50%"
            , style "width" "22px"
            , style "height" "22px"
            , style "font-size" "13px"
            , style "display" "flex"
            , style "align-items" "center"
            , style "justify-content" "center"
            ]
            [ text (String.fromInt count) ]
    else
        text ""

pieceColor : Player -> String
pieceColor player =
    case player of 
        White -> "#f5f0e0"
        Black -> "#515151"

pieceGlyph : PieceKind -> String
pieceGlyph kind =
    case kind of
        Bee -> "🐝"
        Spider -> "🕷"
        Beetle -> "🪲"
        Grasshopper -> "🦗"
        Ant -> "🐜"


-- DECODERS

gameStateDecoder : Decode.Decoder GameState
gameStateDecoder =
    Decode.map5 GameState
        (Decode.field "board" boardDecoder)
        (Decode.field "turn" playerDecoder)
        (Decode.field "turn_number" Decode.int)
        (Decode.field "unplaced" unplacedDecoder)
        (Decode.field "result" (Decode.maybe gameResultDecoder))


boardDecoder : Decode.Decoder Board
boardDecoder =
    Decode.field "stacks"
        (Decode.list
            (Decode.map2 Tuple.pair
                (Decode.index 0 positionDecoder)
                (Decode.index 1 (Decode.list pieceDecoder))
            )
        )
        |> Decode.map Board


unplacedDecoder : Decode.Decoder Unplaced
unplacedDecoder =
    Decode.list
        (Decode.map2 Tuple.pair
            (Decode.index 0 playerDecoder)
            (Decode.index 1
                (Decode.list
                    (Decode.map2 Tuple.pair
                        (Decode.index 0 pieceKindDecoder)
                        (Decode.index 1 Decode.int)
                    )
                )
            )
        )


positionDecoder : Decode.Decoder Position
positionDecoder =
    Decode.map2 Position
        (Decode.field "q" Decode.int)
        (Decode.field "r" Decode.int)


pieceDecoder : Decode.Decoder Piece
pieceDecoder =
    Decode.map2 Piece
        (Decode.field "kind" pieceKindDecoder)
        (Decode.field "owner" playerDecoder)


playerDecoder : Decode.Decoder Player
playerDecoder =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "White" -> Decode.succeed White
                    "Black" -> Decode.succeed Black
                    _ -> Decode.fail ("Unknown player: " ++ str)
            )


pieceKindDecoder : Decode.Decoder PieceKind
pieceKindDecoder =
    Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "Bee" -> Decode.succeed Bee
                    "Spider" -> Decode.succeed Spider
                    "Beetle" -> Decode.succeed Beetle
                    "Grasshopper" -> Decode.succeed Grasshopper
                    "Ant" -> Decode.succeed Ant
                    _ -> Decode.fail ("Unknown piece kind: " ++ str)
            )


gameResultDecoder : Decode.Decoder GameResult
gameResultDecoder =
    Decode.oneOf
        [ Decode.field "Win" playerDecoder |> Decode.map Win
        , Decode.string
            |> Decode.andThen
                (\str ->
                    if str == "Draw" then
                        Decode.succeed Draw
                    else
                        Decode.fail ("Unknown result: " ++ str)
                )
        ]

-- ENCODERS

encodeMove : Move -> Encode.Value
encodeMove move = 
    case move of
        Place kind pos ->
            Encode.object
                [ ( "Place" 
                  , Encode.object
                    [ ( "kind", encodePieceKind kind )
                    , ( "at", encodePosition pos)
                    ]
                )]
        MovePiece from to ->
            Encode.object
                [ ( "Move"
                    , Encode.object
                        [ ( "from", encodePosition from)
                        , ( "to", encodePosition to)
                        ]
                    )
                ]

encodeGameState : GameState -> Encode.Value
encodeGameState state = 
    Encode.object
        [ ( "board", encodeBoard state.board)
        , ( "turn" , encodePlayer state.turn)
        , ( "turn_number", Encode.int state.turnNumber)
        , ( "unplaced", encodeUnplaced state.unplaced)
        , ( "result", Maybe.map encodeGameResult state.result |> Maybe.withDefault Encode.null )
        ]

encodeBoard : Board -> Encode.Value
encodeBoard board =
    Encode.object
        [ ( "stacks"
          , Encode.list
                (\( pos, pieces ) -> Encode.list identity [ encodePosition pos, Encode.list encodePiece pieces ])
                board.stacks
          )
        ]

encodePosition : Position -> Encode.Value
encodePosition pos = 
    Encode.object [ ( "q", Encode.int pos.q ), ( "r", Encode.int pos.r ) ]

encodePiece : Piece -> Encode.Value
encodePiece piece = 
    Encode.object [ ( "kind", encodePieceKind piece.kind ), ( "owner", encodePlayer piece.owner ) ]

encodePlayer : Player -> Encode.Value
encodePlayer player =
    Encode.string (playerLabel player)

encodePieceKind : PieceKind -> Encode.Value
encodePieceKind kind =
    Encode.string
        (case kind of
            Bee -> "Bee"
            Spider -> "Spider"
            Beetle -> "Beetle"
            Grasshopper -> "Grasshopper"
            Ant -> "Ant"
        )

encodeUnplaced : Unplaced -> Encode.Value
encodeUnplaced unplaced =
    Encode.list
        (\( player, kinds ) ->
            Encode.list identity
                [ encodePlayer player
                , Encode.list (\( kind, count ) -> Encode.list identity [ encodePieceKind kind, Encode.int count ]) kinds
                ]
        )
        unplaced

encodeGameResult : GameResult -> Encode.Value
encodeGameResult result =
    case result of
        Win player ->
            Encode.object [ ( "Win", encodePlayer player ) ]

        Draw ->
            Encode.string "Draw"

-- MAIN

main : Program () Model Msg
main =
    Browser.element
        { init = \_ -> ( init, Cmd.none )
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
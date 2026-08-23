#pragma once

#include <array>
#include <optional>
#include <unordered_map>
#include <vector>

#include "engine/Types.h"

namespace hive
{
    class Board 
    {
    public:
        const std::vector<Piece>& stackAt(AxialCoord coord) const;
        std::optional<Piece> topPiece(AxialCoord coord) const;
        bool isEmpty(AxialCoord coord) const;

        void placePiece(AxialCoord coord, Piece piece);
        Piece removeTopPiece(AxialCoord coord);

        std::vector<AxialCoord> occupiedCells() const;

        static std::array<AxialCoord, 6> neighbours(AxialCoord coord);

    private:
        std::unordered_map<AxialCoord, std::vector<Piece>> cells_;
        static const std::vector<Piece> kEmptyStack;
    };
} // Namespace hive

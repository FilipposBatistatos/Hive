#include "engine/Board.h"

namespace hive
{
    const std::vector<Piece> Board::kEmptyStack{};

    /* Return the stack of piece at that location or the piece at that specific location */
    const std::vector<Piece>& Board::stackAt(AxialCoord coord) const
    {
        auto it = cells_.find(coord);
        return it == cells_.end() ? kEmptyStack : it->second;
    }

    std::optional<Piece> Board::topPiece(AxialCoord coord) const
    {
        const auto& stack = stackAt(coord);
        return stack.empty() ? std::nullopt : std::optional<Piece>(stack.back());
    }

    bool board::isEmpty(AxialCoord coord) const
    {
        return stackAt(coord).empty();
    }

    void Board::placePiece(AxialCoord coord, Piece piece)
    {
        cell_[coord].push_back(piece);
    }

    Piece Board::removeTopPiece(AxialCoord coord)
    {
        auto it = cells_.find(coord);
        Piece top = it->second.back();
        it->second.pop_back();
        if (it->second.empty()) cells_.erase(it);
        return top;
    }

    std::vector<AxialCoord> Board::occupiedCells() const
    {
        std::vector<AxialCoord> result;
        result.reserve(cells_.size());
        for (const auto& [coord, stack] : cells_)
        {
            if (!stack.empty())
                result.push_back(coord);
        }
        return result;
    }    

    static std::array<AxialCoord, 6> Board::neighbours(AxialCoord coord)
    {
        std::array<AxialCoord, 6> result{};
        for (int i = 0; i < 6; ++i)
        {
            result[i] = {coord.q + kAxialDirections[i].q, coord.r + kAxialDirections[i].r}
        }
        return result;
    }

} // Namespace hive

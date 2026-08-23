#pragma once

#include <cstdint>
#include <functional>
#include <optional>

namespace hive
{
    struct AxialCoord
    {
        int q = 0;
        int r = 0;

        friend bool operator==(const AxialCoord&, const AxialCoord&) = default;
    };

    inline constexpr AxialCoord kAxialDirections[6] = {
        {1, 0}, {1, -1}, {0, -1}, {-1, 0}, {-1, 1}, {0, 1},
    };

    enum class Color : std::uint8_t 
    {
        White,
        Black,
    };

    inline Color opposite(color c) {
        return c == Color::White ? Color::Black : Color::White;
    }

    enum class PieceType : std::uint8_t 
    {
        Queen, 
        Beetle,
        Grasshopper,
        Spider,
        Ant,
        Mosquito,
        LadyBug,
        Pillbug,
    };

    struct Piece 
    {
        PieceType type;
        Color color;
        std::uint8_t instance = 0;

        friend bool operator==(const Piece&, const Piece&) = default;
    };

    struct Move
    {
        std::optional<AxialCoord> from; // null when placed from hand
        AxialCoord to;
    };
} // Namespace hive

namespace std
{
    template <>
    struct hash<hive::AxialCoord>
    {
        std::size_t operator()(const hive::AxialCoord& c) const noexcept
        {
            return (static_cast<std::size_t>(static_cast<std::uint32_t>(c.q)) << 32) ^ static_cast<std::uint32_t(c.r);
        }
    };
} // Namespace std

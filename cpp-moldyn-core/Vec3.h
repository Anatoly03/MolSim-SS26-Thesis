/**
 * @file Vec3.h
 * @brief Definition of the Vec3 class for three-dimensional vector operations.
 */

#pragma once

#include <cmath>
#include <optional>

/**
 * @brief A struct representing a three-dimensional [mathematical vector](https://en.wikipedia.org/wiki/Vector_%28mathematics_and_physics%29).
 * @tparam T The type of the vector components, defaulting to double.
 */
template <typename T = double>
struct Vec3
{
    // allow numeric types only.
    // https://stackoverflow.com/a/26207551
    static_assert(std::is_arithmetic<T>::value, "Vec3 requires an arithmetic generic type");

public:
    T x;
    T y;
    T z;

    /**
     * @brief Initializes a zero vector.
     */
    Vec3() : x(0), y(0), z(0) {}

    /**
     * @brief Initializes a vector with equal components.
     * @param e The value to set for all components of the vector.
     *
     * # Example
     *
     * ```cpp
     * Vec3<double> a(1.0); // == Vec3(1.0, 1.0, 1.0)
     * Vec3<double> a(-1.0); // == Vec3(-1.0, -1.0, -1.0)
     * ```
     */
    Vec3(T e) : x(e), y(e), z(e) {}

    /**
     * @brief Initializes a vector with specified components.
     */
    Vec3(T x, T y, T z) : x(x), y(y), z(z) {}

    // remove implicit copy constructors
    // https://stackoverflow.com/questions/33776697/deleting-copy-constructors-and-copy-assignment-operators-which-of-them-are-esse
    Vec3(const Vec3 &) = delete;
    // Vec3 &operator=(const Vec3 &) = delete;
    inline constexpr Vec3 clone() const { return Vec3<T>(x, y, z); }

    /**
     * @brief Vec3 dot product.
     * @details Returns the scalar product of this vector with another vector.
     */
    // the type conversion to double is to avoid issues when T is an integer type
    inline constexpr double dot(const Vec3 &other) const
    {
        return double(x) * other.x + double(y) * other.y + double(z) * other.z;
    }

    /**
     * @brief Vec3 cross product.
     */
    // the type conversion to double is to avoid issues when T is an integer type
    inline constexpr Vec3 cross(const Vec3 &other) const
    {
        return Vec3(y * other.z - z * other.y, z * other.x - x * other.z, x * other.y - y * other.x);
    }

    /**
     * @brief Vec3 reduced to the length of 1.
     */
    inline std::optional<Vec3> normal() const
    {
        if (double length = this->length())
        {
            return std::optional(*this / length);
        }

        return std::nullopt;
    }

    /**
     * @brief Retrieve the length of the vector.
     * @see https://de.wikipedia.org/wiki/Euklidische_Norm
     */
    inline constexpr double length() const { return std::sqrt(dot(*this)); }

    /**
     * @brief Retrieve the length squared of the vector in the second norm.
     */
    inline constexpr double length2() const { return dot(*this); }

    // operator overloads
    // allows to override symbols like the plus and minus and use it with vectors
    // https://en.cppreference.com/w/cpp/language/operators.html
    // https://www.geeksforgeeks.org/cpp/how-to-overload-the-plus-operator-in-cpp/

    // arithmetic and logical operators

    /**
     * @brief Unary plus operator overload for Vec3.
     */
    inline constexpr Vec3 operator+() const { return *this; }

    /**
     * @brief Unary minus operator overload for Vec3.
     */
    inline constexpr Vec3 operator-() const { return Vec3(-x, -y, -z); }

    /**
     * @brief Binary addition operator overload for Vec3.
     */
    inline constexpr Vec3 operator+(const Vec3 &other) const { return Vec3(x + other.x, y + other.y, z + other.z); }

    /**
     * @brief Binary subtraction operator overload for Vec3.
     */
    inline constexpr Vec3 operator-(const Vec3 &other) const { return Vec3(x - other.x, y - other.y, z - other.z); }

    /**
     * @brief Binary multiplication operator overload for Vec3 with a scalar.
     */
    inline constexpr Vec3 operator*(const T &scalar) const { return Vec3(x * scalar, y * scalar, z * scalar); }

    /**
     * @brief Binary multiplication operator overload for a scalar with a Vec3.
     * @note This is a friend function as the scalar is on the left.
     */
    inline constexpr friend Vec3 operator*(const T &scalar, const Vec3 &other)
    {
        return Vec3(other.x * scalar, other.y * scalar, other.z * scalar);
    }

    /**
     * @brief Binary division operator overload for Vec3 with a scalar.
     */
    inline constexpr Vec3 operator/(const T &scalar) const { return Vec3(x / scalar, y / scalar, z / scalar); }

    /**
     * @brief Vec3 equivalence.
     */
    inline constexpr bool operator==(const Vec3 &other) const
    {
        return (x == other.x) && (y == other.y) && (z == other.z);
    }

    /**
     * @brief Vec3 inequivalence.
     */
    inline constexpr bool operator!=(const Vec3 &other) const
    {
        return (x != other.x) || (y != other.y) || (z != other.z);
    }

    // assignment operators

    /**
     * @brief Assignment operator overload for Vec3.
     */
    // constexpr Vec3(const Vec3 &other) = default;
    constexpr Vec3 &operator=(const Vec3 &other) = default;

    /**
     * @brief Binary addition operator overload for Vec3.
     */
    inline constexpr Vec3 &operator+=(const Vec3 &other)
    {
        x += other.x;
        y += other.y;
        z += other.z;
        return *this;
    }

    /**
     * @brief Binary subtraction operator overload for Vec3.
     */
    inline constexpr Vec3 &operator-=(const Vec3 &other)
    {
        x -= other.x;
        y -= other.y;
        z -= other.z;
        return *this;
    }

    /**
     * @brief Binary multiplication operator overload for Vec3 with a scalar.
     */
    inline constexpr Vec3 &operator*=(const T &scalar)
    {
        x *= scalar;
        y *= scalar;
        z *= scalar;
        return *this;
    }

    /**
     * @brief Binary division operator overload for Vec3 with a scalar.
     */
    inline constexpr Vec3 &operator/=(const T &scalar)
    {
        x /= scalar;
        y /= scalar;
        z /= scalar;
        return *this;
    }

    // casting

    template <typename K>
    explicit operator Vec3<K>() const
    {
        return Vec3<K>(static_cast<K>(x), static_cast<K>(y), static_cast<K>(z));
    }
};

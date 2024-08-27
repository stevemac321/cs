#include <iostream>
#include <concepts>
#include <type_traits>
#include <bitset>

// Define the BitState enum
enum BitState {
    ZERO_ZERO,
    ZERO_ONE,
    ONE_ONE
};

// Define a concept that checks for unsigned integral types
template<typename T>
concept UnsignedBitwiseOperable = std::is_integral_v<T> && std::is_unsigned_v<T> && requires(T a, T b) {
    { a & b };   // Bitwise AND
    { a | b };   // Bitwise OR
    { a ^ b };   // Bitwise XOR
    { ~a };      // Bitwise NOT
    { a << 1 };  // Left shift
    { a >> 1 };  // Right shift
};


template<UnsignedBitwiseOperable T>
T getbit(T x, size_t pos)
{
  return (x >> pos) & 1;
}
template<UnsignedBitwiseOperable T>
void setbit(T &x, size_t pos, size_t val)
{
  if (val == 1) {
    x |= (T(1) << pos);  // Set the bit at 'position' to 1
  } else {
    x &= ~(T(1) << pos); // Clear the bit at 'position' to 0
  }
}

template<UnsignedBitwiseOperable T>
T addbits(T a, T b) {
    T c = 0;
    bool carry_flag = false;
    size_t sum = 0;

    for (size_t i = 0; i < sizeof(T) * 8; i++) {  // Loop through each bit
        size_t bit_a = getbit(a, i);
        size_t bit_b = getbit(b, i);

        // Determine the bit state
        BitState state = static_cast<BitState>(bit_a + bit_b);

        // Calculate the sum and update carry_flag based on the state
        switch (state) {
            case BitState::ZERO_ZERO:
                sum = carry_flag ? 1 : 0;
                carry_flag = false;
                break;
            case BitState::ZERO_ONE:
                sum = carry_flag ? 0 : 1;
                carry_flag = carry_flag ? true : false;
                break;
            case BitState::ONE_ONE:
                sum = carry_flag ? 1 : 0;
                carry_flag = true;
                break;
        }
	setbit(c, i, sum);
       
    }
    if (carry_flag) {
        setbit(c, (sizeof(T) * 8) - 1, 1);
    }
    return c;
}
int main() {
    uint8_t a8 = 0b00001111; // 15 in decimal
    uint8_t b8 = 0b00110011; // 51 in decimal
    uint8_t result8 = addbits(a8, b8);
    std::cout << "uint8_t Result: " << +result8 << " (Binary: " << std::bitset<8>(result8) << ")" << std::endl;

    // Test with uint16_t
    uint16_t a16 = 0b0000111100001111; // 3855 in decimal
    uint16_t b16 = 0b0011001100110011; // 13107 in decimal
    uint16_t result16 = addbits(a16, b16);
    std::cout << "uint16_t Result: " << result16 << " (Binary: " << std::bitset<16>(result16) << ")" << std::endl;

    // Test with uint32_t
    uint32_t a32 = 0b00001111000011110000111100001111; // 252645135 in decimal
    uint32_t b32 = 0b00110011001100110011001100110011; // 858993459 in decimal
    uint32_t result32 = addbits(a32, b32);
    std::cout << "uint32_t Result: " << result32 << " (Binary: " << std::bitset<32>(result32) << ")" << std::endl;

    // Test with uint64_t
    uint64_t a64 = 0b0000111100001111000011110000111100001111000011110000111100001111ULL; // 1085102592571150095 in decimal
    uint64_t b64 = 0b0011001100110011001100110011001100110011001100110011001100110011ULL; // 3689348814741910323 in decimal
    uint64_t result64 = addbits(a64, b64);
    std::cout << "uint64_t Result: " << result64 << " (Binary: " << std::bitset<64>(result64) << ")" << std::endl;
    return 0;
}


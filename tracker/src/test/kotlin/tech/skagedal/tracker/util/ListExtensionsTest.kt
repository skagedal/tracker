package tech.skagedal.tracker.util

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class ListExtensionsTest {
    @Test
    internal fun `test segment`() {
        val nums: List<Int> = listOf(1, 2, 3, -1, 20, -1, 44)
        val (first, rest) = nums.segment { it != -1 }
        assertEquals(listOf(1, 2, 3), first)
        assertEquals(listOf(-1, 20, -1, 44), rest)

        val (first2, rest2) = rest.drop(1).segment { it != -1 }
        assertEquals(listOf(20), first2)
        assertEquals(listOf(-1, 44), rest2)

        val (first3, rest3) = rest2.drop(1).segment { it != -1 }
        assertEquals(listOf(44), first3)
        assertEquals(emptyList(), rest3)
    }

    @Test
    internal fun `test split sublists`() {
        val nums: List<Int> = listOf(-1, 1, 2, 3, -1, 20, -1, 44)
        assertEquals(
            listOf(
                listOf(-1, 1, 2, 3),
                listOf(-1, 20),
                listOf(-1, 44)
            ),
            nums.splitSublists { it == -1 }
        )
    }
}
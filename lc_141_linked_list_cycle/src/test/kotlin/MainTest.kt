package org.example

import kotlin.test.Test
import kotlin.test.assertEquals

class MainTest {

    /**
     * Builds a linked list from [values]. If [pos] is a valid index into [values],
     * the tail's next pointer is wired back to that node to create a cycle,
     * matching LeetCode's (values, pos) input format.
     */
    private fun buildList(values: List<Int>, pos: Int = -1): ListNode? {
        if (values.isEmpty()) return null

        val nodes = values.map { ListNode(it) }
        for (i in 0 until nodes.size - 1) {
            nodes[i].next = nodes[i + 1]
        }
        if (pos in nodes.indices) {
            nodes.last().next = nodes[pos]
        }
        return nodes[0]
    }

    @Test
    fun `example 1 - cycle at index 1`() {
        val head = buildList(listOf(3, 2, 0, -4), pos = 1)
        assertEquals(true, Solution().hasCycle(head))
    }

    @Test
    fun `example 2 - cycle at index 0`() {
        val head = buildList(listOf(1, 2), pos = 0)
        assertEquals(true, Solution().hasCycle(head))
    }

    @Test
    fun `example 3 - no cycle single node`() {
        val head = buildList(listOf(1), pos = -1)
        assertEquals(false, Solution().hasCycle(head))
    }

    @Test
    fun `empty list has no cycle`() {
        val head = buildList(emptyList(), pos = -1)
        assertEquals(false, Solution().hasCycle(head))
    }

    @Test
    fun `single node self cycle`() {
        val head = buildList(listOf(1), pos = 0)
        assertEquals(true, Solution().hasCycle(head))
    }

    @Test
    fun `no cycle with multiple nodes`() {
        val head = buildList(listOf(1, 2, 3, 4, 5), pos = -1)
        assertEquals(false, Solution().hasCycle(head))
    }

    @Test
    fun `cycle at last node points to itself`() {
        val nodes = listOf(1, 2, 3).map { ListNode(it) }
        nodes[0].next = nodes[1]
        nodes[1].next = nodes[2]
        nodes[2].next = nodes[2]
        assertEquals(true, Solution().hasCycle(nodes[0]))
    }

    @Test
    fun `long list with cycle near the end`() {
        val head = buildList((1..10000).toList(), pos = 9998)
        assertEquals(true, Solution().hasCycle(head))
    }

    @Test
    fun `long list without cycle`() {
        val head = buildList((1..10000).toList(), pos = -1)
        assertEquals(false, Solution().hasCycle(head))
    }
}

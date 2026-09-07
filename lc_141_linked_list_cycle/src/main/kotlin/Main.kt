package org.example

class ListNode(var `val`: Int) {
    var next: ListNode? = null
}

class Solution {
    fun hasCycle(head: ListNode?): Boolean {
        return if (head == null) {
            false
        } else {
            val processedNodes: MutableSet<ListNode> = mutableSetOf()
            var node = head
            while (node?.next != null) {
                if (processedNodes.contains(node)) {
                    return true
                } else {
                    processedNodes += node
                    node = node.next
                }
            }
            false
        }
    }
}

fun main() {

}
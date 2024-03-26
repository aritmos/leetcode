package linkedlist

func ReorderList(head *ListNode) {
	slow := head
	fast := head
	for fast.Next != nil && fast.Next.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}

	secondList := slow.Next
	slow.Next = nil

	a := head
	b := reverseList(secondList)

	for b != nil {
		tmp := a.Next
		a.Next = b
		b = b.Next
		a.Next.Next = tmp
		a = tmp
	}
}

//          a
// 1    2   nil
// | _/ | _/
// 4/   3/  nil
//           b

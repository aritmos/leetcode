package heap

import "testing"

// ["Twitter","postTweet","getNewsFeed","follow","postTweet","getNewsFeed","unfollow","getNewsFeed"]
// [[],        [1,5],      [1],          [1,2],   [2,6],      [1],          [1,2],     [1]]

func TestHeap(t *testing.T) {
	tw := Constructor()
	tw.PostTweet(1, 5)
	nf1 := tw.GetNewsFeed(1)
	tw.Follow(1, 2)
	tw.PostTweet(2, 6)
	nf2 := tw.GetNewsFeed(1)
	tw.Unfollow(1, 2)
	nf3 := tw.GetNewsFeed(1)
	println(nf1, nf2, nf3)
}

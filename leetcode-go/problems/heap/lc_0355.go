package heap

// Design Twitter

import (
	"container/heap"
	"slices"
)

type Twitter struct {
	timeStamp int // we model each new tweet happening in a new timestep
	users     map[int]*User
}

type post struct {
	id        int
	timeStamp int
}
type postList []post // max-heap

// impl sort.Interface
func (pl postList) Len() int           { return len(pl) }
func (pl postList) Less(i, j int) bool { return pl[i].timeStamp < pl[j].timeStamp }
func (pl postList) Swap(i, j int)      { pl[i], pl[j] = pl[j], pl[i] }

// impl heap.Interace
func (pl *postList) Push(x any) { *pl = append(*pl, x.(post)) }
func (pl *postList) Pop() any {
	l := len(*pl)
	lastPost := (*pl)[l-1]
	*pl = (*pl)[:l-1]
	return lastPost
}

type User struct {
	posts     []post
	following []int
}

func NewUser(userId int) *User {
	pl := postList{}
	heap.Init(&pl)
	return &User{posts: pl, following: []int{userId}}
}

func Constructor_0355() Twitter {
	return Twitter{
		users: make(map[int]*User),
	}
}

func (this *Twitter) PostTweet(userId int, tweetId int) {
	user, exists := this.users[userId]
	if !exists {
		newUser := NewUser(userId)
		this.users[userId] = newUser
		user = newUser
	}
	user.posts = append(user.posts, post{tweetId, this.timeStamp})
	this.timeStamp++
}

const NEWSFEED_SIZE int = 10

func (this *Twitter) GetNewsFeed(userId int) []int {
	user, exists := this.users[userId]
	if !exists {
		return []int{}
	}

	latestPosts := postList{}
	heap.Init(&latestPosts)
	for _, followeeId := range user.following {
		followee, exists := this.users[followeeId]
		if !exists {
			panic("newsfeed error: followee id doesn't exist")
		}
		for _, postId := range followee.posts {
			heap.Push(&latestPosts, postId)
			if len(latestPosts) > NEWSFEED_SIZE {
				heap.Pop(&latestPosts)
			}
		}
	}

	// transform heap into ordered list
	newsFeed := make([]int, len(latestPosts))
	for i := len(latestPosts) - 1; i >= 0; i-- {
		newsFeed[i] = heap.Pop(&latestPosts).(post).id
	}
	return newsFeed
}

func (this *Twitter) Follow(followerId int, followeeId int) {
	follower, exists := this.users[followerId]
	if !exists {
		newUser := NewUser(followerId)
		this.users[followerId] = newUser
		follower = newUser
	}
	_, exists = this.users[followeeId]
	if !exists {
		newUser := NewUser(followeeId)
		this.users[followeeId] = newUser
	}

	// only add a followee if it doesnt already exist
	if idx := slices.Index(follower.following, followeeId); idx == -1 {
		follower.following = append(follower.following, followeeId)
	}
}

func (this *Twitter) Unfollow(followerId int, followeeId int) {
	follower, exists := this.users[followerId]
	if !exists {
		panic("unfollow error: follower id does not exist")
	}

	following := follower.following

	idx := slices.Index(following, followeeId)
	if idx == -1 {
		// follower doesnt follow followee, do nothing
		return
	}
	if idx == len(following)-1 {
		follower.following = following[:len(following)-1]
	} else {
		follower.following = append(following[:idx], following[idx+1:]...)
	}
}

/**
 * Your Twitter object will be instantiated and called as such:
 * obj := Constructor();
 * obj.PostTweet(userId,tweetId);
 * param_2 := obj.GetNewsFeed(userId);
 * obj.Follow(followerId,followeeId);
 * obj.Unfollow(followerId,followeeId);
 */

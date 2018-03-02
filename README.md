# EulerLisp

![](https://projecteuler.net/profile/leonrische.png)

## Code Samples

``` clojure
(~>
  (range~ 1 1000)
  (select~ &(or (divides? 3 &1) (divides? 5 &1)))
  sum~
  (println "Solution: "))
```
## Special Syntax

1. `fst`/`rst` instead of `car`/`cdr`, incl. chained versions `ffst`, ... (`cadr`)
2. `~>` pipeline operator, `(~> 1 (+ 2) (* 3)` = `(* 3 (+ 2 1))`
3. lambda shorthand, `&(+ &1 (* &2 2))` = `(fn (a b) (+ a (* b 2)))`
4. streams (lazy iterators), `range~`, `map~`, `select~`, ...

## Advanced Data Structures

_Note: Some of these could (and should) be implemented in the target language
at some later time._

### Priority Queues

* `(make-priority-queue pairs)`, create a (max-)priority queue from a list of
   (element, priority) pairs
* `(make-min-priority-queue pairs)`, create a (min-)priority queue from a list of
   (element, priority) pairs
* `(priority-queue-insert! pq element priority)`, insert a new element
* `(priority-queue-max pq)`, get the (element, priority) pair with the highest
   (or lowest, if the queue is a min-priority queue) priorty
* `(priority-queue-pop! pq)`, same as `priority-queue-max`, but removes the element

``` clojure
>> (def pq (make-priority-queue (list (cons "foo" 1) (cons "bar" 5) (cons "baz" 7))))
>> (priority-queue-insert! pq "qux" 20)
>> (priority-queue-max pq)
=> ("qux" . 20)
>> (priority-queue-pop! pq)
=> ("qux" . 20)
>> (priority-queue-pop! pq)
=> ("baz" . 7)
>> (priority-queue-pop! pq)
=> ("bar" . 5)
>> (priority-queue-pop! pq)
=> ("foo" . 1)
```

## Notation for Solution Filenames

* `00.scm` Solution that runs in < 60s
* `00_T.scm` TODO, Solution that is gives the correct answer in < 60s,
  but I'm not sure if the math behind it is always valid
* `00_S.scm` SLOW, Solution that is correct but to slow
* `00_W.scm` WIP, Work-in-progress

## Problems for Performance Testing

* 14, runs in ~22s
* 23, runs in ~62s

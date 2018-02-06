; Solved: 5.2.2018

; Start with something like
; full + (sqrt(n) - rest)/d
;
; Seems like I got it right by chance,
; not 100% sure what is going on here
(defn continue_ (n full rest d)
  (let* ([den (- n (* rest rest))]
         [dv (gcd den d)]
         [d_ (div den dv)]
         [full_ (floor (/ d (- (sqrt n) rest)))]
         [rest_ (- (* full_ d_) rest)])
    (list n full_ rest_ d_)))

(defn continue (l)
      (continue_ (fst l) (frst l) (frrst l) (frrrst l)))

(defn get-initial (n)
  (let ([full (floor (sqrt n))])
    (list n full full 1)))

; Floyd's Cycle Detection Algorithm
(defn find-in-cycle_ (f turtoise hare)
      (if (= turtoise hare)
          turtoise
          (find-in-cycle_
            f
            (f turtoise)
            (f (f hare)))))
(defn find-in-cycle (f initial)
      (find-in-cycle_
        f
        (f initial)
        (f (f initial))))

(defn find-cycle-len_ (f turtoise hare len)
      (if (= turtoise hare)
          len
          (find-cycle-len_ f turtoise (f hare) (inc len)))) 
(defn find-cycle-len (f turtoise)
      (find-cycle-len_ f turtoise (f turtoise) 1))
(defn cycle-len (f initial)
      (find-cycle-len f (find-in-cycle f initial)))

(~>
  (range~ 1 10_000)
  (select~ &(not (square? &1)))
  (map~ &(cycle-len continue (get-initial &1)))
  (count~ odd?)
  solution)

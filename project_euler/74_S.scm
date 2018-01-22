; Solved: 15.1
; Time 7:06
; Solution: 402

(def facs (map fac (range 0 9)))

(defn first-step (n)
      (reduce (fn (d acc)
                  (+ acc (list-ref facs d)))
              0
              (number->digits n)))

(def steps '(1))
(defn loop (cur)
  (when (< cur 10000)
        (push! steps (first-step cur))
        (loop (inc cur))))

(loop 1)
(println "done preprocessing")

(defn step (n)
  (if (< n 10000)
      (list-ref steps n)
      (reduce (fn (d acc)
                  (+ acc (list-ref facs d)))
              0
              (number->digits n))))

; Floyd's Cycle Detection Algorithm
(defn find-in-cycle_ (turtoise hare)
      (if (= turtoise hare)
        turtoise
        (find-in-cycle_
          (step turtoise)
          (step (step hare)))))
(defn find-in-cycle (n)
      (find-in-cycle_
        (step n)
        (step (step n))))

(defn find-cycle-len_ (turtoise hare len)
      (if (= turtoise hare)
        len
        (find-cycle-len_ turtoise (step hare) (inc len)))) 
(defn find-cycle-len (turtoise)
      (find-cycle-len_ turtoise (step turtoise) 1))

(defn find-cycle-start_ (turtoise hare mu)
      (if (= turtoise hare)
          mu
          (find-cycle-start_ (step turtoise) (step hare) (inc mu))))
(defn find-cycle-start (n in-cycle)
      (find-cycle-start_ n in-cycle 0))

(defn before-cycle-len (n)
    (let (in (find-in-cycle n))
      (+
        (find-cycle-start n in)
        (find-cycle-len in))))

(def lens '(1))
(defn solve (from (count 0))
      (println "from = " from)
      (if (> from 1000000)
        count
        (let (len (before-cycle-len from))
          (push! lens len)
          (if (= len 60)
            (do
              (println "found: " from)
              (solve (inc from) (inc count)))
            (solve (inc from) count)))))

(println "Solution: " (solve 1))

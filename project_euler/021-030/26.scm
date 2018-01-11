; Solved 5.1

(defn make-stepper (d)
  (fn (n)
    (% (* 10 n) d)))
(def initial 1)

; Floyd's Cycle Detection Algorithm
(defn find-in-cycle (f)
      (find-in-cycle_
        f
        (f initial)
        (f (f initial))))
(defn find-in-cycle_ (f turtoise hare)
      (if (= turtoise hare)
          turtoise
          (find-in-cycle_
            f
            (f turtoise)
            (f (f hare)))))

; We don't need this right now
(defn find-cycle-start (f in-cycle)
      (find-cycle-start_ f initial in-cycle 0))
(defn find-cycle-start_ (f turtoise hare mu)
      (if (= turtoise hare)
          mu
          (find-cycle-start_ f (f turtoise) (f hare) (inc mu)))) 

(defn find-cycle-len (f turtoise)
      (find-cycle-len_ f turtoise (f turtoise) 1))

(defn find-cycle-len_ (f turtoise hare len)
      (if (= turtoise hare)
          len
          (find-cycle-len_ f turtoise (f hare) (inc len)))) 

(defn cycle-len (f)
      (find-cycle-len f (find-in-cycle f)))

(defn loop (from to max-n max-len)
      (if (> from to)
        max-n
        (let*
          (f (make-stepper from)
           len (cycle-len f))
          (if (> len max-len)
              (loop (inc from) to from len)
              (loop (inc from) to max-n max-len)))))

(println "Solution: " (loop 2 1000 0 0))

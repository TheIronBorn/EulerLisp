; Solved 5.1.2018

(defn make-stepper (d) &(% (* 10 &1) d))
(def initial 1)

; Floyd's Cycle Detection Algorithm
(defn find-in-cycle_ (f turtoise hare)
      (if (= turtoise hare)
          turtoise
          (find-in-cycle_
            f
            (f turtoise)
            (f (f hare)))))
(defn find-in-cycle (f)
      (find-in-cycle_
        f
        (f initial)
        (f (f initial))))

; We don't need this right now
(defn find-cycle-start_ (f turtoise hare mu)
      (if (= turtoise hare)
          mu
          (find-cycle-start_ f (f turtoise) (f hare) (inc mu)))) 
(defn find-cycle-start (f in-cycle)
      (find-cycle-start_ f initial in-cycle 0))

(defn find-cycle-len_ (f turtoise hare len)
      (if (= turtoise hare)
          len
          (find-cycle-len_ f turtoise (f hare) (inc len)))) 
(defn find-cycle-len (f turtoise)
      (find-cycle-len_ f turtoise (f turtoise) 1))

(defn cycle-len (f)
      (find-cycle-len f (find-in-cycle f)))

(defn loop (from (max-n 0) (max-len 0))
      (if (> from 1000)
        max-n
        (let*
          ([f (make-stepper from)]
           [len (cycle-len f)])
          (if (> len max-len)
              (loop (inc from) from len)
              (loop (inc from) max-n max-len)))))

(solution (loop 2))

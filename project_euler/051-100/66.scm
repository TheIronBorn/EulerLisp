; Solved 6.2.2018
;
; Note:
; * https://en.wikipedia.org/wiki/Pell%27s_equation#The_smallest_solution_of_Pell_equations

; Code copied from problem 64
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

; Fix until bignum subtraction is implemented
(defn solves? (n num den)
  (= (* num num)
     (+ 1 (* n den den))))
; (defn solves? (n num den)
;   (= 1 (- (* num num)
;           (* n den den))))

(defn step_ (s_prev num_prev num den_prev den)
  (if (solves? (fst s_prev) num den)
      (cons num den)
      (let ([s (continue s_prev)])
        (let ([num_new (+ (* (frst s) num) num_prev)]
              [den_new (+ (* (frst s) den) den_prev)])
          (step_ s num num_new den den_new)))))

(defn step (n)
  (let ([initial (get-initial n)])
    (step_
      initial
      1 (frst initial)
      0 1
      )))

(defn solve (to (d 1) (max-d 0) (max-x 0))
  (cond
    [(> d to) max-d]
    [(square? d) (solve to (inc d) max-d max-x)]
    [else
      (let ([s (step d)])
        (if (> (fst s) max-x)
          (solve to (inc d) d (fst s))
          (solve to (inc d) max-d max-x)))]))

(solution (solve 1000))

; Solved 28.1.18
;
; The first version was correct in theory
; but would have taken years to solve the problem for high numbers.
; Rewriting it in crystal reduced the time to 15min
; and submitting the solution unlocked a long pdf
; with proofs of better algorithms.
;
; The code below is an implementation of Algorithm 6
; from the pdf. Somewhere hidden in there is some error
; (might be an integer over / underflow) so the direct result is wrong,
; but multiplying and subtracting it to get the final solution
; seems to fix that.


(def n 500_000_000)
(def l (isqrt n))
(def v (filled-list (inc l) 0))
(def floor-nl (div n l))
(def big-v (filled-list (inc floor-nl) 0))

(defn inner1-1 (x g to res)
      (if (> g to) res
        (inner1-1 x (inc g) to (- res (list-ref v (div x g))))))
(defn inner1-2 (x z to res)
      (if (> z to) res
        (inner1-2 x (inc z) to
                  (if (= z (div x z)) res
                    (- res (* (- (div x z) (div x (inc z)))
                              (list-ref v z)))))))

(defn loop1 (x)
      (when (<= x l)
        (~>
          (gauss-sum x)
          (inner1-1 x 2 (isqrt x))
          (inner1-2 x 1 (isqrt x))
          (set-nth! v x))
        (loop1 (inc x))))

(defn inner2-1 (x k g to res)
      (if (> g to) res
        (inner2-1 x k (inc g) to
                  (if (<= (div k g) l)
                    (- res (list-ref v (div k g)))
                    (- res (list-ref big-v (* x g)))))))
(defn inner2-2 (x k z to res)
      (if (> z to) res
        (inner2-2 x k (inc z) to
                  (if (= z (div k z)) res
                    (- res (* (- (div k z) (div k (inc z)))
                              (list-ref v z)))))))

(defn loop2 (x)
      (when (> x 0)
        (let* ([k (div n x)])
          (~>
            (gauss-sum k)
            (inner2-1 x k 2 (isqrt k))
            (inner2-2 x k 1 (isqrt k))
            (set-nth! big-v x)))
        (loop2 (dec x))))

(loop1 1)
(loop2 floor-nl)

(~> (list-ref big-v 1) (* 6) (- (* n (inc n) 3)) (println "Solution: "))

;; Nice and short version that works well for n < 10M,
;; this is an implementation of Algorithm 3.2 from the pdf
; 
; (def tots (filled-list (inc n) 0))
; (defn fill (cur)
;   (when (<= cur n)
;     (set-nth! tots cur cur)
;     (fill (inc cur))))
; (defn update (p k n)
;   (when (<= k n
;     (let ([old (list-ref tots k)])
;       (set-nth! tots k (- old (div old p))))
;     (update p (+ k p) n)))
; (defn loop (p acc)
;   (if (<= p n)
;       (do
;         (if (= p (list-ref tots p)) (update p p n))
;           (loop
;             (inc p)
;             (+ acc (- p (list-ref tots p)))))
;     acc))
; (fill 0)
; (~> (loop 2 0)
;     (* 6)
;     (println "Solution: "))

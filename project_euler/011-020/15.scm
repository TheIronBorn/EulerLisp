; Solved 1.1

(def known '())

; Assuming a < b
(defn index (a b)
    (dec
      (+
        (/ (* b (dec b)) 2)
        a)))

(defn iterate (a b maxa maxb)
  (if (> a b)
      (if (= b maxb)
          'done
          (iterate 1 (inc b) maxa maxb))
      (do
        (push! known (paths a b))
        (iterate (inc a) b maxa maxb))))

(defn paths (a b)
  (if (> a b)
      (paths b a)
      (cond
        ((= a 1) (inc b))
        ((< (index a b) (length known))
         (list-ref known (index a b)))
        (else
          (+ (paths (dec a) b)
             (paths a (dec b)))))))

(iterate 1 1 20 20)
(println "Solution: " (nth (index 20 20) known))

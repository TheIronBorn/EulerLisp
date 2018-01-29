; Solved: 28.1.18

(defn rects (a b) (* (/ (* a (inc a)) 2)
                     (/ (* b (inc b)) 2)))

(defn find-best (a b)
  (let ([r (rects a b)])
    (if (> r 2000000)
      (let
        ([s1 (abs (- 2000000 (rects a (dec b))))]
         [s2 (abs (- r 2000000))]) 
        (if (< s1 s2)
            (list s1 (* a (dec b)))
            (list s2 (* a b))))
      (find-best a (inc b)))))

(~>
  (range 1 2000)
  (map &(find-best &1 &1))
  (min-by fst)
  frst
  (println "Solution: "))

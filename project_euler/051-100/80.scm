; Solved: 8.2.2018

(defn find-x (p c (x 0))
  (if (> (* (inc x) {{20 * p} + (inc x)}) c)
    x
    (find-x p c (inc x))))

(defn root-digits (p c prec (acc 0))
  (let* ([x (find-x p c)]
         [y (* x (+ (* 20 p) x))]
         [p_ (+ (* 10 p) x)]
         [c_ (* (- c y) 100)])
    (if (> prec 0)
      (root-digits p_ c_ (dec prec) (+ acc x))
      acc)))

(~>
  (range~ 1 100)
  (select~ &(not (square? &1)))
  (map~ &(root-digits 0 &1 100))
  sum~
  solution)

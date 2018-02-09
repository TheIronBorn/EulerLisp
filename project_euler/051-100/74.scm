; Solved: 15.1.2018

(def facs (map fac (range 0 9)))
(defn step (n)
      (reduce &(+ &2 (list-ref facs &1))
              0
              (number->digits n)))

(def lens '(1 1))
(def length-lens 2)

(defn solve (n (acc 0))
  (if (< n length-lens)
      (+ acc (list-ref lens n))
      (case n
        [145 (+ acc 1)]
        [169 (+ acc 3)]
        [1454 (+ acc 3)]
        [363601 (+ acc 3)]
        [871 (+ acc 2)]
        [45361 (+ acc 2)]
        [872 (+ acc 2)]
        [45362 (+ acc 2)]
        [else
          (let ([next (step n)])
            (if (= next n)
                (inc acc)
                (solve next (inc acc))))])))

(def result 0)

(defn fill (n (acc 0))
  (if (>= n 1000000)
    acc
    (let ([len (solve n)])
      (push! lens len)
      (set! length-lens (inc length-lens))
      (fill (inc n) (if (= len 60) (inc acc) acc)))))

(solution (fill 2))

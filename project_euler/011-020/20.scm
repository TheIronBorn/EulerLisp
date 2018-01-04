; Solved 1.1

(def digits #(1))

(defn multiply (n) (multiply-helper n 0 (length digits) 0))

(defn multiply-helper (n from to carry)
    (if (= from to)
        (if (> carry 0)
          (do
            (vector-push! digits (% carry 10))
            (multiply-helper n from to (/ carry 10))))
        (let ((cur (+ (* n (nth from digits)) carry)))
             (vector-set! digits from (% cur 10))
             (multiply-helper n (inc from) to (/ cur 10)))))

(defn multiply-n (n)
      (if (> n 0)
          (do
            (multiply n)
            (multiply-n (dec n)))))

(defn vector-sum (from to acc)
      (if (= from to)
          acc
          (vector-sum
            (inc from)
            to
            (+ acc (nth from digits)))))

(multiply-n 100)
(println (vector-sum 0 (length digits) 0))

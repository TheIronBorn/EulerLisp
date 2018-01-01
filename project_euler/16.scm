; Solved 1.1

(def digits #(1))

(defn double () (double-helper 0 (length digits) 0))

(defn double-helper (from to carry)
    (if (= from to)
        (if (> carry 0)
          (do
            (vector-push! digits (% carry 10))
            (double-helper from to (/ carry 10))))
        (let ((cur (+ (* 1024 (nth from digits)) carry)))
             (vector-set! digits from (% cur 10))
             (double-helper (inc from) to (/ cur 10)))))

(defn double-n (n)
      (if (> n 0)
          (do
            (double)
            (double-n (dec n)))))

(defn vector-sum (from to acc)
      (if (= from to)
          acc
          (vector-sum
            (inc from)
            to
            (+ acc (nth from digits)))))

(double-n (/ 1000 10))
(println (vector-sum 0 (length digits) 0))

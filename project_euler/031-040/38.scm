; Solved 8.1

(defn member? (e lst)
  (cond
    (nil? list) #f
    (= (fst lst) e) #t
    else (member? e (rst lst))))


(defn pandigital? (lst)
  (= (sort lst)
     (list 1 2 3 4 5 6 7 8 9)))

(defn all-digits (lst)
      (flatmap digits lst))

(defn solve (n x max-pan)
  (let* ((ds (all-digits (map (fn (a) (* x a)) (range 1 n)))))
    (if (> (length ds) 9)
      (if (= n 9)
          max-pan
          (solve (inc n) 1 max-pan))
      (if (pandigital? ds)
          (solve n (inc x)
                 (max max-pan
                      (string->number (apply str ds))))
          (solve n (inc x) max-pan)))))

(println "Solution: " (solve 2 1 0))

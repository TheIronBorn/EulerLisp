(defn multiple (a b) (= (% a b) 0))

(defn solve (n)
  (solve_ n 0 0))

(defn solve_ (limit n acc)
  (if (> n limit)
      acc
      (if (and (not (multiple n 15))
               (or (multiple n 5)
                   (multiple n 3)))
        (solve_ limit (+ n 1) (+ acc n))
        (solve_ limit (+ n 1) acc))))

(puts (solve 1000))

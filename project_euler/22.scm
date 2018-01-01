; Solved 1.1
; Changes:
;  * add thread macro `~>`

(def names
     (~> "project_euler/22.txt"
         file-read
         (string-split ",")
         sort))

(defn value (name)
      (sum
        (map (fn (x)
                 (if (and (>= x 65) (<= x 90))
                     (- x 64)
                     0))
          (string-bytes name))))

(defn solve (names index acc)
      (if (empty? names)
          acc
          (solve
            (rst names)
            (inc index)
            (+ acc (* index (value (fst names)))))))

(println (solve names 1 0))

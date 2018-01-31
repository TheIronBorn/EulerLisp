; Solved 1.1
; Changes:
;  * add thread macro `~>`
;  * use new char-* functions

(defn value (name)
      (reduce-sum
        &(if (char-alphabetic? &1) (- (char->integer &1) 64) 0)
        (string->chars name)))

(defn solve (names (index 1) (acc 0))
      (if (empty? names)
        acc
        (solve
          (rst names)
          (inc index)
          (+ acc (* index (value (fst names)))))))

(~> "project_euler/input-files/22.txt"
    file-read
    (string-split ",")
    sort
    solve
    (println "Solution: "))

; Solved 1.1
; Changes:
;  * add `file-read` function

(def input
  (map (fn (x)
           (map string->number (words x)))
      (~> "project_euler/18.txt"
          file-read
          lines
          reverse
          rst)))

(defn reduce-rows (a b acc)
  (if (nil? b)
      (reverse acc)
      (reduce-rows
        (rst a)
        (rst b)
        (cons
          (max
            (+ (fst a) (fst b))
            (+ (frst a) (fst b)))
          acc))))

(defn reduce-full (input)
    (if (= 1 (length input))
        (ffst input)
        (reduce-full
          (cons
            (reduce-rows (fst input) (frst input) '())
            (rrst input)))))

(println (reduce-full input))

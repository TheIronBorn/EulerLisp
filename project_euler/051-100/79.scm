; Solved 22.1.18

(def tries
     (~> "./project_euler/input-files/79.txt"
         file-read
         lines
         (reject empty?)
         (map string->number)
         (map number->digits)))

(defn matches? (try passcode)
  (cond
    [(nil? try) #t]
    [(nil? passcode) #f]
    [(= (fst try) (fst passcode))
      (matches? (rst try) (rst passcode))]
    [else (matches? try (rst passcode))]))

(def all-digits
     (~> (reduce append '() tries) sort uniq))

(println "Are there tries with repeating digits? "
         (any? (fn (try)
                   (< (~> try sort uniq length)
                      (length try)))
               tries
               ))

; This is not guaranteed to work in every case,
; (e.g. tries 12 and 21)
; but good enough to get the solution here
(~>
  (permutations~ all-digits)
  (select~ (fn (code) (all? &(matches? &1 code) tries)))
  collect
  (map digits->number)
  (println "Solutions: "))

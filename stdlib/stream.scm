(defn stream-rst (s) (force (rst s)))

(defn stream-print (limit stream)
  (if (> limit 0)
      (do
        (println (fst stream))
        (stream-print (dec limit) (stream-rst stream)))))

(defn stream-nth (n s)
      (if (= n 0)
          (fst s)
          (stream-nth
            (dec n)
            (stream-rst s))))

(defn stream-map (fun stream)
  (stream-cons
    (fun (fst stream))
    (stream-map fun (stream-rst stream))))

(defn stream-filter (pred stream)
  (if (pred (fst stream))
      (stream-cons
        (fst stream)
        (stream-filter pred (stream-rst stream)))
      (stream-filter
         pred
         (stream-rst stream))))

(defn stream-constant (n)
      (stream-cons
        n
        (stream-constant n)))

(defn stream-combine (fun s1 s2)
  (stream-cons
    (fun (fst s1) (fst s2))
    (stream-combine fun (stream-rst s1) (stream-rst s2))))

(defn stream-add (s1 s2)
  (stream-combine + s1 s2))

(defn natural-numbers-from (n)
  (stream-cons
    n
    (natural-numbers-from (inc n))))

(def natural-numbers (natural-numbers-from 1))

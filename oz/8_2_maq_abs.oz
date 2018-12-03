local A B C in
   thread
        if A then
          B = true
        else
          B = false
        end
   end

   thread
        if B then
          C = false
        else
          C = true
	end
   end
      A = false
end

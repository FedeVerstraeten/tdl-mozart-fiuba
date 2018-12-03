local WaitSome Bound A B in
   proc {WaitSome Xs}
      {ForAll Xs proc {$ X}
            thread
                {Wait X}
                Bound = true
	       {Browse X}
	    end
        end
      }
      {Wait Bound}
      %{Delay 1000}
   end

% ejecucion
    {WaitSome [1 A 3 4 5 B]}
   A = 2
   B = 6
end

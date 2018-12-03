local NewPrintAgent NewFilterAgent Filter SendIntMsg SendSpamMsg in
  
   % Creacion agente que imprime por pantalla
   fun {NewPrintAgent}
      local S in
        thread
           for Msg in S do
              {Browse Msg}
           end
        end
      {NewPort S}
      end
   end

   % Creador del filtro pasando agente como parametro
   % Validador del mensaje
   fun {NewFilterAgent PrintAgent Fun}
      local S in
        thread
           for Msg in S do
              if {Fun Msg} then
                {Send PrintAgent 'Msg accepted : '#Msg}
              else
                {Send PrintAgent 'Msg rejected : '#Msg}
              end
           end
        end
        {NewPort S}
      end
   end          

   % Sender mensajes validos de Integer
   proc {SendIntMsg Port}
      proc {Repeat}
        {Delay {OS.rand} mod 1000}
        {Send Port {OS.rand}}
        {Repeat}
      end
   in
      thread {Repeat} end
   end

   % Sender mensajes validos de String Spam
   proc {SendSpamMsg Port}
      proc {Repeat}
        {Delay {OS.rand} mod 1000}
        {Send Port 'spam'}
        {Repeat}
      end
   in
      thread {Repeat} end
   end

   % Creacion filtro
   Filter = {NewFilterAgent {NewPrintAgent} IsInt}

   % El servidor de filtro recibe mensajes desde dos hilos
   % con distintos tipos de mensaje

   {SendIntMsg Filter}
   {SendSpamMsg Filter}
end
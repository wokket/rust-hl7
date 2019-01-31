using System;
using System.Runtime.InteropServices;

namespace ConsoleApp1
{
    public class MessageHandle :SafeHandle
    {

        /// <param name="ptr">A pointer returned from a call to BuildMessage</param>
        [DllImport("rusthl7", CallingConvention = CallingConvention.Cdecl, EntryPoint = "free_message")]
        private static extern void FreeMessage(IntPtr ptr);

        public MessageHandle() : base(IntPtr.Zero, true) { }
        public override bool IsInvalid => false;

        protected override bool ReleaseHandle()
        {
            FreeMessage(handle);
            return true;
        }
    }
}

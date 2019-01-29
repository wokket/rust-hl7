using System;
using System.Runtime.InteropServices;
using System.Text;

namespace ConsoleApp1
{

    /// <summary>
    /// Provides for safe consumption and cleanup of CString values returned from the Native code.
    /// </summary>
    public class StringHandle : SafeHandle
    {

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "free_string")]
        private static extern void FreeString(IntPtr ptr);


        public StringHandle() : base(IntPtr.Zero, true) { }
        public override bool IsInvalid => false;

        protected override bool ReleaseHandle()
        {
            FreeString(handle);
            return true;
        }

        public string AsString()
        {
            int len = 0;
            while (Marshal.ReadByte(handle, len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }
    }
}

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.011.002 Put Project Data Servers</name>
   <tag></tag>
   <elementGuidId>f61095f5-ce4c-4958-baa1-0d9bb3758c75</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;domainNameServer\&quot;: \&quot;asdf.id\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>982f5b01-e4ac-4773-a53d-6812c4f2db30</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>00809c0f-06f5-4306-b112-ef46f4ff34d6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${project}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.project_api_servers</defaultValue>
      <description></description>
      <id>4abeb5d5-b73d-4f5a-89ec-d8df99f1e140</id>
      <masked>false</masked>
      <name>project</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>5ac007c3-5e55-40c4-81a8-8899c02881c6</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'project/project-detail/66d52629da1f6ebdaa8e6243/api-server/data-server'</defaultValue>
      <description></description>
      <id>31ba1f9a-376c-46e8-8d03-a9fdb25a7581</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
